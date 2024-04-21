use actix_web::{middleware, web, App, HttpServer};

mod config;
use crate::config::SERVER_URL;
use crate::config::DATABASE_URL;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //链接数据库
    let pool = sqlx::MySqlPool::connect(DATABASE_URL).await.unwrap();
    HttpServer::new(move || {
        App::new()
    })
        .bind(SERVER_URL)?
        .run()
        .await
}