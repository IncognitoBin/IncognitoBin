use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use scylla::{Session, SessionBuilder};
use crate::config::settings::Config;

mod db;
mod models;
mod routes;
mod handlers;
mod config;
mod utils;

use crate::db::init::initialize_schema;
use crate::db::scylla_db_operations::ScyllaDbOperations;
use crate::routes::api;

struct RedisAppState {
    redis_client: Arc<redis::Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ENV Config
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    // Scylla Connection
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1")
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");
    // Init Scylla DB
    if let Err(err) = initialize_schema(&session, "resources/init.sql").await {
        eprintln!("Failed to initialize schema: {:?}", err);
    }
    // Scylla db Operations Handler
    let db_ops = web::Data::new(ScyllaDbOperations::new(Arc::new(session).clone()));

    // Redis Connection
    let redis_client = Arc::new(redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to open Redis client"));
    let redis_app_state = web::Data::new(RedisAppState {
        redis_client: redis_client.clone(),
    });
    println!("Starting Now");

    HttpServer::new(move || {
        App::new()
            .app_data(db_ops.clone())
            .app_data(web::Data::new(config.clone()).clone())
            .app_data(redis_app_state.clone())
            .configure(api::configure)
    })
        .bind(("0.0.0.0", 8181))?
        .run()
        .await
}