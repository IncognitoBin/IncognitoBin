
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::{ScyllaDbOperations};
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use crate::{RedisAppState};
use crate::config::settings::Config;
use crate::utils::helpers::{extract_user_token, number_text_to_uuid};
use crate::db::redis_operations::dequeue;
use crate::models::user::UserById;
use crate::models::user_vm::{CreatedUserResponse, UserLoginRequest, UserLoginResponse};

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
        user_id,
        user_token: "x".to_string(),
    };
    match db.insert_user_by_id(&user).await {
        Ok(_) => HttpResponse::Ok().json(CreatedUserResponse{id:user_id}),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
#[post("/user")]
async fn user_login(
    db: web::Data<ScyllaDbOperations>,
    login_data: web::Json<UserLoginRequest>,
    redis_con: web::Data<RedisAppState>,
) -> impl Responder {
    let user_old_token = match db.get_user_by_id(login_data.id).await {
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
    db.execute_update_token_operations(user_old_token,new_user_token.clone(),&login_data.id).await.unwrap();
    HttpResponse::Ok().json(UserLoginResponse {token:new_user_token})
}
#[delete("/user")]
async fn user_logout(
    db: web::Data<ScyllaDbOperations>,
    req: HttpRequest,
    config: web::Data<Config>
) -> impl Responder {
    let token: Option<String>;
    token = match extract_user_token(&req, &config).await {
        Some(token) => Some(token.to_string()),
        _ => {return HttpResponse::Unauthorized().finish();}

    };
    db.delete_user_token(token.unwrap().to_string()).await.unwrap();
    HttpResponse::Ok().finish()
}