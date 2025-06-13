use crate::error::ApiError;
use crate::db;
use rand::Rng;
use rand::distr::Alphanumeric;
use std::string::String;

fn generate_random_string(length: usize) -> String {
    let s: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    s
}

pub async fn register_user(username: &str, password: &str, email: &str, ip: &str) -> Result<String, ApiError> {
    let pool = db::get_db_pool()?;    
    let token = generate_random_string(32);
    
    let _row = sqlx::query("INSERT INTO users (username, password, email, token, last_login_ip) VALUES (?, ?, ?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(email)
        .bind(token.clone()) // 克隆 token
        .bind(ip)
        .execute(pool).await?;
    
    Ok(token)
}