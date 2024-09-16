use crate::config::Config;
use crate::db::models::PasteById;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;
use crate::view_model::models::{CreatePasteRequest, CreatePasteResponse};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
struct PasteResponse {
    paste: PasteById,
    views: i64,
}
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}
#[get("/paste/{paste_id}")]
async fn get_paste(
    db: web::Data<ScyllaDbOperations>,
    paste_id: web::Path<Uuid>,
) -> impl Responder {
    let paste_id = paste_id.into_inner();

    let paste_result = db.get_paste_by_id(paste_id).await;
    db.increment_view_count_by_paste_id(paste_id).await.expect("Can't Increment Views");
    let view_count_result = db.get_view_count_by_paste_id(paste_id).await;
    match (paste_result, view_count_result) {
        (Ok(Some(paste)), Ok(Some(views))) => {
            let response = PasteResponse {
                paste,
                views: views.0,
            };
            HttpResponse::Ok().json(response)
        }
        (Ok(None), _) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

async fn generate_unique_id() -> Result<Uuid> {
    let response = reqwest::get("http://localhost:8080/id")
        .await
        .context("Failed to connect to ID generation service")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("ID generation service returned an error: {}", response.status()));
    }

    let id: u128 = response.text().await
        .context("Failed to read response from ID generation service")?
        .parse()
        .context("Failed to parse ID as u128")?;

    Ok(Uuid::from_u128(id))
}
fn is_valid_bcrypt_hash(hash: String) -> Option<u8> {
    if hash.len() != 60 {
        return None;
    }
    if hash.starts_with("$2b$") {
        let parts: Vec<&str> = hash.split('$').collect();
        if parts.len() > 2 {
            if let Ok(cost) = parts[2].parse::<u8>() {
                return Some(cost);
            }
        }
    }
    None
}
#[post("/paste")]
async fn create_paste(
    req: HttpRequest,
    db: web::Data<ScyllaDbOperations>,
    config: web::Data<Config>,
    paste_data: web::Json<CreatePasteRequest>,
) -> impl Responder {
    // Title
    if paste_data.title.len() > config.max_title_length as usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Title must not exceed {} characters", config.max_title_length).to_string() });
    }
    // Content Size
    if paste_data.content.len() > config.max_content_kb as usize || paste_data.content.len() < 24 {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Content size must be between 24 AND {} bytes", config.max_content_kb).to_string() });
    }
    // Expiration
    let expiration_time = match paste_data.expire {
        Some(seconds) => {
            if seconds == 0 {
                None
            } else if seconds >= 10 * 60 && seconds <= 365 * 24 * 60 * 60 {
                Some(Utc::now() + Duration::seconds(seconds))
            } else {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Expiration time must be between 10 minutes and 1 year".to_string(),
                });
            }
        }
        None => None,
    };
    // UserID
    let mut user_id: Option<Uuid> = None;
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_token) = auth_header.to_str() {
            if auth_token.len() == config.token_size as usize {
                let get_user_token = db.get_userid_by_token(auth_token).await;
                if get_user_token.is_ok() {
                    user_id = get_user_token.unwrap_or(None);
                }
            }
        }
    }
    // Password
    if paste_data.password.is_some() {
        match is_valid_bcrypt_hash(paste_data.password.clone().unwrap_or("".to_string())) {
            Some(cost) => {
                if cost != config.bcrypt_rounds {
                    return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Bcrypt Rounds Must Be {} ", config.bcrypt_rounds).to_string() });
                }
            }
            None => return HttpResponse::BadRequest().json(ErrorResponse { error: "Invalid bcrypt hash.".to_string() }),
        }
    }
    // Syntax
    if paste_data.syntax.is_some() && paste_data.syntax.clone().unwrap_or("".to_string()).len() > config.max_syntax_length as usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Syntax must not exceed {} characters", config.max_syntax_length).to_string() });
    }
    // Get Unique ID
    let paste_id = match generate_unique_id().await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to generate unique ID: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    // Save In DB
    let new_paste = PasteById {
        paste_id,
        title: paste_data.title.clone(),
        content: paste_data.content.clone(),
        syntax: paste_data.syntax.clone(),
        password: paste_data.password.clone(),
        encrypted: paste_data.encrypted,
        expire: expiration_time,
        burn: paste_data.burn.unwrap_or(false),
        user_id,
    };
    match db.insert_paste(&new_paste).await {
        Ok(_) => {
            if let Some(user_id) = new_paste.user_id {
                if let Err(e) = db.insert_paste_by_user_id(user_id, paste_id).await {
                    eprintln!("Failed to associate paste with user: {:?}", e);
                }
            }
            HttpResponse::Created().json(CreatePasteResponse { paste_id })
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}