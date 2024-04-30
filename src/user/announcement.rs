use actix_web::Result;
use sqlx::Error;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;
use crate::models::Announcement;

pub async fn get_announcement() -> Result<Announcement, Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;

    sqlx::query_as::<_, Announcement>("SELECT * FROM announcements ORDER BY created_time DESC LIMIT 1")
        .fetch_one(&pool)
        .await
}