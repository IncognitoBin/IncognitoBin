use std::thread;
use std::time::Duration;
use actix_web::{get, put, web, App, HttpResponse, HttpServer, Responder};
use crate::chunks::{add_id, retrieve_id};

mod chunks;
const SPLIT_SIZE: u16 = 30000;
#[get("/id")]
async fn get_id() -> impl Responder {
    format!("{}",retrieve_id())
}
#[put("/id/{id_str}")]
async fn put_id(id_str: web::Path<String>) -> impl Responder {
    let id = match id_str.parse::<u128>() {
        Ok(value) => value,
        Err(_) => 0, // Convert parse error to 0
    };

    if id == 0 {
        HttpResponse::BadRequest().finish() // Return a complete BadRequest response
    } else {
        add_id(id); // Use the parsed ID
        HttpResponse::Ok().finish() // Return a complete Ok response
    }
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match chunks::load() {
        Ok(()) => println!("Data loaded successfully!"),
        Err(e) => eprintln!("Failed to load data: {}", e),
    }

    thread::spawn(|| {
        loop{
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
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}