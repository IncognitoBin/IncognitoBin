use actix_web::{get, put, web, HttpResponse, Responder};
use crate::AppState;
use crate::paste_ids::{add_id};
use crate::redis_handler::dequeue;

#[get("/id")]
pub async fn get_id(data: web::Data<AppState>) -> impl Responder {
    let mut con = data.redis_client.get_connection()
        .expect("Failed to get Redis connection");
    match dequeue(&mut con, "paste_ids") {
        Ok(Some(id)) => HttpResponse::Ok().body(id),
        Ok(None) => HttpResponse::NotFound().body("No IDs available"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Redis error: {}", e)),
    }
}

#[put("/id/{id_str}")]
pub async fn put_id(id_str: web::Path<String>) -> impl Responder {
    let id = id_str.parse::<u128>().unwrap_or(0);
    if id == 0 {
        HttpResponse::BadRequest().finish()
    } else {
        add_id(id);
        HttpResponse::Ok().finish()
    }
}
