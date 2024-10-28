use std::env;
use std::sync::Arc;
use std::time::Duration;
use dotenv::dotenv;
use scylla::{Session, SessionBuilder};

mod paste_ids;
mod db;
mod utils;
mod handlers;
mod config;

use crate::db::db_operations_imlp::ScyllaDbOperations;
use crate::handlers::paste_ids::pastes_ids_handler;
use crate::handlers::user_auth::{ids_queue_handler, tokens_queue_handler};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let scylla_host: String = env::var("SCYLLA_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let redis_host: String = env::var("REDIS_HOST").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let session: Session = SessionBuilder::new()
        .known_node(scylla_host)
        .build()
        .await
        .expect("Failed to connect to ScyllaDB");
    let session = Arc::new(session);
    let client = Arc::new(redis::Client::open(redis_host)?);
    paste_ids::load()?;
    println!("Data loaded successfully!");

    // Store In Json
    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(15)).await;
            if let Err(e) = paste_ids::store_chunks() {
                eprintln!("Can't Store The File: {}", e);
            }
        }
    });

    // Add Paste IDs To Queue
    let redis_paste_ids_client = client.clone();
    tokio::spawn(async move {
        let con = redis_paste_ids_client.get_connection()
            .expect("Failed to get Redis connection for paste_ids");
        pastes_ids_handler(con).await;
    });

    // Add Tokens To Queue
    let redis_users_tokens_client = client.clone();
    let scylla_users_tokens_session = session.clone();
    tokio::spawn(async move {
        let con = redis_users_tokens_client.get_connection()
            .expect("Failed to get Redis connection for users_tokens");
        let db_ops = ScyllaDbOperations::new(scylla_users_tokens_session);
        tokens_queue_handler(con, db_ops).await;
    });

    // Add UsersId To Queue
    let users_tokens_client = client.clone();
    let scylla_users_ids_session = session.clone();
    tokio::spawn(async move {
        let con = users_tokens_client.get_connection()
            .expect("Failed to get Redis connection for users_tokens");
        let db_ops = ScyllaDbOperations::new(scylla_users_ids_session);
        ids_queue_handler(con, db_ops).await;
    });
    println!("Starting Now");

    // Keep the main thread running
    tokio::signal::ctrl_c().await?;
    println!("Shutting down");
    Ok(())
}