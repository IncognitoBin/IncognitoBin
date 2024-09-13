use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use scylla::{Session, SessionBuilder};
mod api;
mod db;

use crate::api::{get_paste};
use crate::db::init::initialize_schema;
use crate::db::scylla_db_operations::ScyllaDbOperations;

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
    let db_ops = web::Data::new(ScyllaDbOperations::new(session.clone()));

    println!("Starting Now");
    HttpServer::new(move || {
        App::new()
            .app_data(db_ops.clone())
            .service(get_paste)
    })
        .bind(("0.0.0.0", 8181))?
        .run()
        .await
}
