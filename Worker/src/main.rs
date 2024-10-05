use std::sync::Arc;
use std::thread;
use std::time::Duration;
use actix_web::{web, App, HttpServer};
use scylla::{Session, SessionBuilder};

mod paste_ids;
mod api;
mod redis_handler;
mod db;
mod user_auth;

use crate::api::{get_paste_id,get_user_id,get_user_token, put_id};
use crate::db::db_operations_iml::ScyllaDbOperations;
use crate::paste_ids::handlers::pastes_ids_handler;
use crate::user_auth::handlers::{ids_queue_handler, tokens_queue_handler};

struct AppState {
    redis_client: Arc<redis::Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session: Session = SessionBuilder::new()
        .known_node("127.0.0.1")
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");
    let session = Arc::new(session);

    let client = Arc::new(redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to open Redis client"));

    match paste_ids::load() {
        Ok(()) => println!("Data loaded successfully!"),
        Err(e) => eprintln!("Failed to load data: {}", e),
    }

    // Store In Json
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(15));
            paste_ids::store_chunks().expect("Can't Store The File!");
        }
    });

    // Add Paste IDs To Queue
    let redis_paste_ids_client = client.clone();
    tokio::spawn(async move {
        let con = redis_paste_ids_client.get_connection().expect("Failed to get Redis connection for paste_ids");
        pastes_ids_handler(con).await;
    });

    // Add Tokens To Queue
    let redis_users_tokens_client = client.clone();
    let scylla_users_tokens_session = session.clone();
    tokio::spawn(async move {
        let con = redis_users_tokens_client.get_connection().expect("Failed to get Redis connection for users_tokens");
        let db_ops = ScyllaDbOperations::new(scylla_users_tokens_session);
        tokens_queue_handler(con, db_ops).await
    });

    // Add UsersId To Queue
    let users_tokens_client = client.clone();
    let scylla_users_ids_session = session.clone();
    tokio::spawn(async move {
        let con = users_tokens_client.get_connection().expect("Failed to get Redis connection for users_tokens");
        let db_ops = ScyllaDbOperations::new(scylla_users_ids_session);
        ids_queue_handler(con, db_ops).await;
    });

    let app_state = web::Data::new(AppState {
        redis_client: client.clone(),
    });
    println!("Starting Now");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_paste_id)
            .service(get_user_id)
            .service(get_user_token)
            .service(put_id)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}