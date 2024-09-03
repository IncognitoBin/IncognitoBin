use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Paste {
    id: u32,
    content: String,
}
#[get("/paste")]
pub async fn get_paste() -> impl Responder {
    let paste = Paste {
        id: 1,
        content: "X".to_string(),
    };
    web::Json(paste)
}

#[post("/paste")]
pub async fn create_paste(paste: web::Json<Paste>) -> impl Responder {
    HttpResponse::Ok().json(paste.into_inner())
}

#[delete("/paste")]
pub async fn remove_paste() -> impl Responder {
    format!("{}", 1)
}
