use actix_web::{get, put, web, HttpResponse, Responder};
use crate::chunks::{add_id, retrieve_id};

#[get("/id")]
pub async fn get_id() -> impl Responder {
    format!("{}", retrieve_id())
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
