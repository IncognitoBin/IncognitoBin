use std::env;
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use scylla::{Session, SessionBuilder};
use crate::config::settings::Config;
use actix_cors::Cors;

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

    let scylla_host: String = env::var("SCYLLA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let session: Session = SessionBuilder::new()
        .known_node(scylla_host)
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
    let redis_host: String = env::var("REDIS_HOST").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_client = Arc::new(redis::Client::open(redis_host)
        .expect("Failed to open Redis client"));
    let redis_app_state = web::Data::new(RedisAppState {
        redis_client: redis_client.clone(),
    });
    println!("Starting Now");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive() // This allows all origins, methods, and headers
            )
            .app_data(db_ops.clone())
            .app_data(web::Data::new(config.clone()).clone())
            .app_data(redis_app_state.clone())
            .configure(api::configure)
    })
        .bind(("0.0.0.0", 8181))?
        .run()
        .await
}