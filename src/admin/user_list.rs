use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

use crate::models::UserInfo;

/// Initialize the database connection pool from the environment configuration
async fn get_db_pool() -> Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    dotenv().ok(); // Load environment variables from a .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new().connect(&database_url).await
}

pub async fn user_list(token: &str, page: i32) -> Result<Vec<UserInfo>, sqlx::Error> {
    let pool = get_db_pool().await?;
    let is_admin: i32 = sqlx::query_scalar(
        "SELECT is_admin FROM users WHERE token = ?"
    )
    .bind(token)
    .fetch_one(&pool)
    .await?;

    if is_admin == 1 {
        // Calculate the correct offset for pagination
        let offset = (page - 1) * 10;
        let user_list: Vec<UserInfo> = sqlx::query_as::<_, UserInfo>(
            "SELECT * FROM users LIMIT ?, 10"
        )
        .bind(offset)
        .fetch_all(&pool)
        .await?;
        Ok(user_list)
    } else {
        Err(sqlx::Error::RowNotFound) // Better error to describe permission issue?
    }
}
