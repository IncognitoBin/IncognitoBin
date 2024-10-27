
use crate::Config;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::{ScyllaDbOperations};
use crate::models::paste_vm::{CreatePasteRequest, CreatedPasteResponse, GetPasteGenInfo, PasteResponse};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::{RedisAppState};
use crate::utils::helpers::{extract_user_id, number_text_to_uuid, time_difference_in_seconds};
use crate::db::redis_operations::dequeue;
use crate::models::paste::PasteById;

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
                signature: paste.signature,
                content: paste.content,
                syntax: paste.syntax,
                expire: time_difference_in_seconds(paste.expire),
                views: 0,
            };
            // Burn
            if paste.burn {
                if paste.user_id != None {
                    db.delete_paste_by_user_id(&paste_id,&paste.user_id.unwrap()).await.expect("Can't Delete The Paste");
                }else{
                    db.delete_paste_by_id(&paste_id).await.expect("Can't Delete The Paste");
                }
            }else{
                // Increment
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
#[get("/paste")]
async fn get_user_pastes(
    req: HttpRequest,
    config: web::Data<Config>,
    db: web::Data<ScyllaDbOperations>,
) -> impl Responder {
    // User Id extraction with error handling
    let user_id = match extract_user_id(&req, &db, &config).await {
        Some(id) => id,
        None => {
            return HttpResponse::Unauthorized().body("Unauthorized: Invalid user credentials")
        }
    };

    // Fetch all paste IDs for the user
    let pastes = match db.get_pastes_by_userid(user_id).await {
        Ok(pastes_uuid) => pastes_uuid,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .body("Internal server error while retrieving pastes")
        }
    };

    // Gather information for each paste, with improved error handling
    let mut user_pastes: Vec<GetPasteGenInfo> = Vec::new();
    for uuid in &pastes {
        let paste = match db.get_paste_info_by_id(uuid.clone()).await {
            Ok(Some(paste_info)) => paste_info,
            Ok(None) => {
                return HttpResponse::NotFound().body("Paste not found")
            }
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Error retrieving paste information")
            }
        };

        let views = match db.get_view_count_by_paste_id(uuid.clone()).await {
            Ok(Some(views)) => views.0,
            Ok(None) => 0i64, // No views recorded, default to 0
            Err(_) => {
                return HttpResponse::InternalServerError()
                    .body("Error retrieving view count")
            }
        };

        let new_paste_info = GetPasteGenInfo {
            id: uuid.clone(),
            burn: paste.burn,
            expire: time_difference_in_seconds(paste.expire),
            views,
        };
        user_pastes.push(new_paste_info);
    }

    // Return the JSON response with collected paste data
    HttpResponse::Ok().json(user_pastes)
}

#[post("/paste")]
async fn create_paste(
    req: HttpRequest,
    db: web::Data<ScyllaDbOperations>,
    redis_con: web::Data<RedisAppState>,
    config: web::Data<Config>,
    paste_data: web::Json<CreatePasteRequest>,
) -> impl Responder {
    // Title
    if paste_data.title.len() > config.max_title_length as usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Title must not exceed {} bytes", config.max_title_length).to_string() });
    }
    // Content Size
    if paste_data.content.len() > config.max_content_kb as usize || paste_data.content.len() < 24 {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Content size must be between 24 and {} bytes", config.max_content_kb).to_string() });
    }
    let mut duration = 0;
    // Expiration
    let expiration_time = match paste_data.expire {
        Some(seconds) => {
            if seconds == 0 {
                None
            } else if seconds >= config.min_paste_duration && seconds <= config.max_paste_duration {
                duration=seconds;
                Some(Utc::now() + Duration::seconds(seconds as i64))
            } else {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: "Expiration time must be between 1 minute and 1 year".to_string(),
                });
            }
        }
        None => None,
    };
    // UserID
    let user_id: Option<Uuid>;
    user_id = match extract_user_id(&req, &db, &config).await {
        Some(id) => Option::from(id),
        _ => {None}
    };
    // Signature
    if paste_data.signature.len() != 24usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: "invalid Signature".to_string() });
    }
    // Syntax
    if paste_data.syntax.is_some() && paste_data.syntax.clone().unwrap_or("".to_string()).len() > config.max_syntax_length as usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Syntax must not exceed {} bytes", config.max_syntax_length).to_string() });
    }
    // Get Unique ID
    let mut con = match redis_con.redis_client.get_connection() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let text_paste_id_num = match dequeue(&mut con, "paste_ids") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().body("No IDs in queue"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let paste_id = number_text_to_uuid(text_paste_id_num);
    // Save In DB
    let new_paste = PasteById {
        paste_id,
        title: paste_data.title.clone(),
        signature: paste_data.signature.clone(),
        content: paste_data.content.clone(),
        syntax: paste_data.syntax.clone(),
        expire: expiration_time,
        burn: paste_data.burn.unwrap_or(false),
        user_id,
    };
    match db.insert_paste(&new_paste, duration).await {
        Ok(_) => {
            if let Some(user_id) = new_paste.user_id {
                if let Err(e) = db.insert_paste_by_user_id(user_id, paste_id,duration).await {
                    eprintln!("Failed to associate paste with user: {:?}", e);
                }
            }
            HttpResponse::Created().json(CreatedPasteResponse { id:paste_id })
        }
        Err(_) => {
            HttpResponse::InternalServerError().finish()
        }
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