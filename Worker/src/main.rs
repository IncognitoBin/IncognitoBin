use std::sync::Arc;
use std::thread;
use std::time::{Duration};
use actix_web::{web, App, HttpServer};

mod chunks;
mod api;
mod redis_handler;

use crate::api::{get_id, put_id};
use crate::redis_handler::{queue_length, setup_enqueue};
struct AppState {
    redis_client: Arc<redis::Client>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(redis::Client::open("redis://127.0.0.1/")
        .expect("Failed to open Redis client"));
    let mut con = client.get_connection().expect("Failed to get Redis connection");
    match chunks::load() {
        Ok(()) => println!("Data loaded successfully!"),
        Err(e) => eprintln!("Failed to load data: {}", e),
    }

    // Store In Json
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(15));
            chunks::store_chunks().expect("Can't Store The File!");
        }
    });
    // Add To Queue
    thread::spawn(move || {
        loop {
            let length = queue_length(&mut con, "paste_ids").expect("Redis: Failed to get queue count");
            if length<1_000_000 {
                setup_enqueue(&mut con,"paste_ids",1_000_000-length).expect("Redis: Can't Insert To Queue");
            }
            thread::sleep(Duration::from_secs(5));
        }
    });
    let app_state = web::Data::new(AppState {
        redis_client: client.clone(),
    });
    println!("Starting Now");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_id)
            .service(put_id)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}