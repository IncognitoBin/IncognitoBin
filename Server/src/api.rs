use crate::config::Config;
use crate::db::models::{PasteById, UserById};
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;
use crate::view_model::models::{CreatePasteRequest, CreatePasteResponse};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::helpers::{extract_user_id, generate_unique_id};

#[derive(Serialize)]
struct PasteResponse {
    title: String,
    content: String,
    syntax: Option<String>,
    expire: Option<DateTime<Utc>>,
    password: bool,
    username:Option<String>,
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
    match db.get_paste_by_id(paste_id).await {
        Ok(Some(paste)) => {
            // Expiration
            if paste.expire.is_some(){
                if paste.expire.unwrap()<Utc::now(){
                    if paste.user_id != None {
                        db.delete_paste_by_user_id(&paste_id,&paste.user_id.unwrap()).await.expect("Can't Delete The Paste");
                    }else{
                        db.delete_paste_by_id(&paste_id).await.expect("Can't Delete The Paste");
                    }
                    return HttpResponse::NotFound().finish();
                }
            }
            let mut response = PasteResponse {
                title: paste.title,
                content: paste.content,
                syntax: paste.syntax,
                expire: paste.expire,
                password: paste.password,
                username:None,
                views: 0,
            };
            // Username
            if paste.user_id.is_some(){
                match db.get_user_by_id(paste.user_id.unwrap()).await {
                    Ok(Some(user)) => {response.username=Some(user.username)}
                    Err(_) => { return HttpResponse::InternalServerError().finish()}
                    _ => {}
                }
            }
            // Burn
            if paste.burn {
                if paste.user_id != None {
                    db.delete_paste_by_user_id(&paste_id,&paste.user_id.unwrap()).await.expect("Can't Delete The Paste");
                }else{
                    db.delete_paste_by_id(&paste_id).await.expect("Can't Delete The Paste");
                }
            }else{// Increment

                db.increment_view_count_by_paste_id(paste_id).await.expect("Cant' Increment Views");
                // Views
                match db.get_view_count_by_paste_id(paste_id).await {
                    Ok(Some(views)) => {
                        response.views= views.0;
                    }
                    Ok(None) => {
                        return HttpResponse::InternalServerError().finish()
                    }
                    Err(_) => {
                        return HttpResponse::InternalServerError().finish()
                    }
                }
            }



            HttpResponse::Ok().json(response)
        }
        Ok(None) => HttpResponse::NotFound().finish(), // Paste not found
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
    }
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
        password: paste_data.password,
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
#[delete("/paste/{paste_id}")]
async fn delete_paste(
    req: HttpRequest,
    db: web::Data<ScyllaDbOperations>,
    config: web::Data<Config>,
    paste_id:web::Path<Uuid>
) -> impl Responder {
    // PasteID
    let paste_id = paste_id.into_inner();
    // UserID
    let user_id = match extract_user_id(&req, &db, &config).await {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().json(ErrorResponse { error: "Invalid token".to_string() }),
    };
    // Check + Delete
    match db.check_paste_by_userid(&user_id, &paste_id).await {
        Ok(true) => match db.delete_paste_by_user_id(&paste_id, &user_id).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Ok(false) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/*
#[get("/user")]
async fn new_user(
    req: HttpRequest,
    db: web::Data<ScyllaDbOperations>,
    config: web::Data<Config>,
) -> Box<dyn Responder> {
    // RateLimit
    // Get Random ID
    let user_uuid = Uuid::new_v4();
    let user = UserById {
        user_id: Uuid::new_v4(), // Generate a new UUID
        user_token: "".to_string(),
        username: "".to_string(),
    };
    match db.insert_user_by_id(&user).await {
        Ok(_) => {HttpResponse::Ok().json(user_uuid)}
        Err(_) => {HttpResponse::ServiceUnavailable()}
    }
*/

