use actix_web::{get, web, HttpResponse, Responder};
use serde::{Serialize};
use uuid::Uuid;
use crate::db::models::PasteById;
use crate::db::paste_db_operations::PasteDbOperations;
use crate::db::scylla_db_operations::ScyllaDbOperations;

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