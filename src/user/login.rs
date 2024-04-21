use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use crate::config::DATABASE_URL;

#[tokio::main]
async fn user_login_get_token(username: &str, password: &str) -> Result<String, sqlx::Error> {
    let pool = MySqlPoolOptions::new().connect(DATABASE_URL).await?;
    
    let row = sqlx::query("SELECT * FROM user WHERE username = ? AND password = ?")
        .bind(username)
        .bind(password)
        .fetch_one(&pool).await?;
    
    // 从数据库返回用户token
    let token: String = row.try_get("token")?;
    Ok(token)
}