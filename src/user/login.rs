use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;
use crate::models::UserLogin;

pub async fn user_login_get_token(username: &str, password: &str) -> Result<UserLogin, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new().connect(&database_url).await?;

    sqlx::query_as!(
        UserLogin,
        "SELECT token FROM users WHERE username = ? AND password = ?",
        username,
        password
    )
    .fetch_one(&pool)
    .await
}