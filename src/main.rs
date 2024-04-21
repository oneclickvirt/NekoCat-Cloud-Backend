use actix_web::{middleware, web, App, HttpServer};
use std::io;

mod config;
use config::SERVER_URL;

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(move || {
        App::new()
    })
        .bind(SERVER_URL)?
        .run()
        .await
}