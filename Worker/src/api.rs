use actix_web::{get, put, web, HttpResponse, Responder};
use crate::AppState;
use crate::paste_ids::{add_id};
use crate::redis_handler::{get_id_from_redis};
#[get("/paste/id")]
pub async fn get_paste_id(data: web::Data<AppState>) -> impl Responder {
    get_id_from_redis(&data, "paste_ids")
}

#[put("/paste/id/{id_str}")]
pub async fn put_id(id_str: web::Path<String>) -> impl Responder {
    match id_str.parse::<u128>() {
        Ok(id) if id > 0 => {
            add_id(id);
            HttpResponse::Ok().finish()
        },
        _ => HttpResponse::BadRequest().finish(),
    }
}

#[get("/user/id")]
pub async fn get_user_id(data: web::Data<AppState>) -> impl Responder {
    get_id_from_redis(&data, "users_ids")
}

#[get("/user/token")]
pub async fn get_user_token(data: web::Data<AppState>) -> impl Responder {
    get_id_from_redis(&data, "users_tokens")
}