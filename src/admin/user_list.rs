use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

use crate::models::UserInfo;
pub async fn user_list(token: &str, page: i32) -> Result<UserInfo, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new().connect(&database_url).await?;
    
    let is_admin: String = sqlx::query_scalar("SELECT is_admin FROM users WHERE token = ?")
        .bind(token)
        .fetch_one(&pool)
        .await?;

    if is_admin == "1" {
        let user_list = sqlx::query_as::<_, UserInfo>("SELECT * FROM users LIMIT ?, 10")
            .bind(page)
            .fetch_one(&pool)
            .await?;
        Ok(user_list)
    } else {
        Err(sqlx::Error::RowNotFound)
    }

}