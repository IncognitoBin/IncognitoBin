use std::sync::Arc;
use actix_web::{web, App, HttpServer};
mod api;
use scylla::{Session, SessionBuilder};
mod db;
mod view_model;
mod config;
mod helpers;
mod redis_handler;

use config::Config;
use crate::api::{create_paste, delete_paste, get_paste, get_user_pastes, new_user, user_login};
use crate::db::init::initialize_schema;
use crate::db::scylla_db_operations::ScyllaDbOperations;

struct RedisAppState {
    redis_client: Arc<redis::Client>,
}

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
    let redis_client = Arc::new(redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to open Redis client"));
    let session = Arc::new(session);
    let db_ops = web::Data::new(ScyllaDbOperations::new(session.clone()));
    let config = match Config::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };
    let redis_app_state = web::Data::new(RedisAppState {
        redis_client: redis_client.clone(),
    });
    println!("Starting Now");
    HttpServer::new(move || {
        App::new()
            .app_data(db_ops.clone())
            .app_data(web::Data::new(config.clone()).clone())
            .app_data(redis_app_state.clone())
            .service(get_paste)
            .service(get_user_pastes)
            .service(create_paste)
            .service(delete_paste)
            .service(new_user)
            .service(user_login)
    })
        .bind(("0.0.0.0", 8181))?
        .run()
        .await
}