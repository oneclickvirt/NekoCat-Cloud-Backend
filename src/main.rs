use actix_web::{middleware, web, App, HttpServer};

mod config;
use crate::config::SERVER_URL;
use crate::config::DATABASE_URL;

mod user;
use user::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(api::web_status)
    })
        .bind(SERVER_URL)?
        .run()
        .await
}