use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use crate::config::DATABASE_URL;

pub async fn get_announcement() -> Result<String, sqlx::Error> {
    let pool = MySqlPoolOptions::new().connect(DATABASE_URL).await?;
    
    let row = sqlx::query("SELECT * FROM announcement")
        .fetch_one(&pool).await?;
    
    // 从数据库返回公告
    let announcement: String = row.try_get("content")?;
    Ok(announcement)
}