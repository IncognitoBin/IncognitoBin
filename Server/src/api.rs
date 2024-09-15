use actix_web::{get, post, web, HttpResponse, Responder};
use anyhow::{Result, Context};
use chrono::{DateTime, Duration, Utc};
use serde::{Serialize};
use uuid::Uuid;
use crate::db::models::PasteById;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;
use crate::view_model::models::{CreatePasteRequest, CreatePasteResponse};

#[derive(Serialize)]
struct PasteResponse {
    paste: PasteById,
    views: i64,
}

#[get("/paste/{paste_id}")]
async fn get_paste(
    db: web::Data<ScyllaDbOperations>,
    paste_id: web::Path<Uuid>
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
        },
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
fn validate_and_convert_expire(expire_seconds: Option<i64>) -> Option<DateTime<Utc>> {
    expire_seconds.and_then(|seconds| {
        if seconds == 0 {
            None
        } else if seconds >= 10 * 60 && seconds <= 365 * 24 * 60 * 60 {
            Some(Utc::now() + Duration::seconds(seconds))
        } else {
            None
        }
    })
}
#[post("/paste")]
async fn create_paste(
    db: web::Data<ScyllaDbOperations>,
    paste_data: web::Json<CreatePasteRequest>,
) -> impl Responder {
    let paste_id = match generate_unique_id().await {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Failed to generate unique ID: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let expire = validate_and_convert_expire(paste_data.expire);
    if(paste_data.title.len()>20){
        return HttpResponse::BadRequest().finish();
    }
    if(paste_data.content.len()>5000000 || paste_data.content.len()<1){
        return HttpResponse::BadRequest().finish();
    }
    // TODO: Syntax Length Check
    // TODO: Passwrod Encryption is BCRYPT
    // TODO: UserID Is Valid
    let new_paste = PasteById {
        paste_id,
        title: paste_data.title.clone(),
        content: paste_data.content.clone(),
        syntax: paste_data.syntax.clone(),
        password: paste_data.password.clone(),
        encrypted: paste_data.encrypted,
        expire,
        burn: paste_data.burn.unwrap_or(false),
        user_id: paste_data.user_id,
    };

    match db.insert_paste(&new_paste).await {
        Ok(_) => {
            if let Some(user_id) = new_paste.user_id {
                if let Err(e) = db.insert_paste_by_user_id(user_id, paste_id).await {
                    eprintln!("Failed to associate paste with user: {:?}", e);
                }
            }

            HttpResponse::Created().json(CreatePasteResponse { paste_id })
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}