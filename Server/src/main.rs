use actix_web::{App, HttpServer};
use actix_web::web::delete;

mod api;
use crate::api::{create_paste, get_paste, remove_paste};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
