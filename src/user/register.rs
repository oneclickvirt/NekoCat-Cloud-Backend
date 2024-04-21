use sqlx::mysql::MySqlPool;
use crate::config::DB_URL;  // 引入 DB_URL

pub async fn register_user(username: &str, password: &str, email: &str, ip: &str) -> Result<(), sqlx::Error> {
    // 连接数据库
    let pool = MySqlPool::connect(DATABASE_URL).await?;

    // 插入用户
    sqlx::query!("INSERT INTO users (username, password, email, ip) VALUES (?, ?, ?, ?)", username, password, email, ip)
        .execute(&pool)
        .await?;

    Ok(())
}