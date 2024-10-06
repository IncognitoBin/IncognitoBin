
use crate::config::Config;
use crate::db::models::{PasteById, UserById};
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::{ScyllaDbOperations};
use crate::view_model::models::{CreatePasteRequest, CreatePasteResponse, UserLoginRequest};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use uuid::Uuid;
use crate::{RedisAppState};
use crate::helpers::{extract_user_id, number_text_to_uuid};
use crate::redis_handler::dequeue;

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
    redis_con: web::Data<RedisAppState>,
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
    let user_id: Option<Uuid>;
    user_id = match extract_user_id(&req, &db, &config).await {
        Some(id) => Option::from(id),
        _ => {None}
    };
    // Syntax
    if paste_data.syntax.is_some() && paste_data.syntax.clone().unwrap_or("".to_string()).len() > config.max_syntax_length as usize {
        return HttpResponse::BadRequest().json(ErrorResponse { error: format!("Syntax must not exceed {} characters", config.max_syntax_length).to_string() });
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

#[get("/user")]
async fn new_user(
    db: web::Data<ScyllaDbOperations>,
    redis_con: web::Data<RedisAppState>,
) -> impl Responder {
    // Get Unique ID
    let mut con = match redis_con.redis_client.get_connection() {
        Ok(conn) => conn,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Redis connection error: {}", e)),
    };
    let text_user_id_num = match dequeue(&mut con, "users_ids") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().body("No IDs in queue"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let user_id = number_text_to_uuid(text_user_id_num);
    let user = UserById {
        user_id: user_id, // Generate a new UUID
        user_token: "x".to_string(),
        username: "".to_string(),
    };
    match db.insert_user_by_id(&user).await {
        Ok(_) => HttpResponse::Ok().json(user_id),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[post("/user")]
async fn user_login(
    db: web::Data<ScyllaDbOperations>,
    login_data: web::Json<UserLoginRequest>,
    redis_con: web::Data<RedisAppState>,
) -> impl Responder {
    let user_id=Uuid::from_u128(login_data.user_id);
    let user_old_token = match db.get_user_by_id(user_id).await {
        Ok(user) => {
            if user.is_none(){
                return HttpResponse::NotFound().finish()
            }
            user.unwrap().user_token
        }
        Err(_) => {return HttpResponse::InternalServerError().finish()}
    };
    // Get New Token
    let mut con = match redis_con.redis_client.get_connection() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let new_user_token = match dequeue(&mut con, "users_tokens") {
        Ok(Some(id)) => id,
        Ok(None) => return HttpResponse::NotFound().body("No IDs in queue"),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    db.execute_update_token_operations(user_old_token,new_user_token.clone(),&user_id).await.unwrap();
    HttpResponse::Ok().json(new_user_token)
}