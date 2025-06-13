use actix_web::{App, HttpServer};
use std::env;
use dotenv::dotenv;

mod user;
use user::api;
mod admin;
use admin::admin_api;
mod models;
mod error;
mod db;
// mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init_db_pool().await.expect("Database pool init failed");
    let server_url = env::var("SERVER_URL").expect("SERVER_URLS must be set");
    HttpServer::new(move || {
        App::new()
        .service(api::web_status)
        .service(api::web_login)
        .service(api::web_register)
        .service(api::web_announcement)
        .service(api::get_ip)
        .service(api::web_cart)
        .service(admin_api::web_login)
        .service(admin_api::get_user_list)
    })
        .bind(server_url)?
        .run()
        .await
}