use std::sync::Arc;
use actix_web::{App, HttpServer};
use scylla::{Session, SessionBuilder};
use uuid::Uuid;

mod api;
mod db;

use crate::api::{create_paste, get_paste, remove_paste};
use crate::db::init::initialize_schema;
use crate::db::queries::{ get_paste_by_id, get_pastes_by_userid, get_view_count_by_paste_id, increment_view_count_by_paste_id};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1")
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");
    if let Err(err) = initialize_schema(&session, "resources/init.sql").await {
        eprintln!("Failed to initialize schema: {:?}", err);
    }
    let session = Arc::new(session);

    println!("Starting Now");
    HttpServer::new(|| {
        App::new()
            .service(get_paste)
            .service(create_paste)
            .service(remove_paste)
    })
        .bind(("0.0.0.0", 8181))?
        .run()
        .await
}
