use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;
use tokio; // Add tokio crate

#[tokio::main]
pub async fn get_announcement() -> Result<String, sqlx::Error> {
    dotenv().ok();
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&DATABASE_URL).await?;
    
    let row = sqlx::query("SELECT * FROM announcement")
        .fetch_one(&pool).await?;
    
    // 从数据库返回公告
    let announcement: String = row.try_get("content")?;
    Ok(announcement)
}