use actix_web::{middleware, web, App, HttpServer};

mod config;
use crate::config::SERVER_URL;

mod user;
use user::api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
        .service(api::web_status)
        .service(api::web_login)
        .service(api::web_register)
        .service(api::web_announcement)
    })
        .bind(SERVER_URL)?
        .run()
        .await
}