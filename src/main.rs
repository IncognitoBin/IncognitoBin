use std::thread;
use std::time::Duration;
use actix_web::{App, HttpServer};
mod chunks;
mod api;
use crate::api::{get_id, put_id};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match chunks::load() {
        Ok(()) => println!("Data loaded successfully!"),
        Err(e) => eprintln!("Failed to load data: {}", e),
    }

    thread::spawn(|| {
        loop {
            chunks::store_chunks().expect("Can't Store The File!");
            thread::sleep(Duration::from_secs(15));
        }
    });

    println!("Starting Now");
    HttpServer::new(|| {
        App::new()
            .service(get_id)
            .service(put_id)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}