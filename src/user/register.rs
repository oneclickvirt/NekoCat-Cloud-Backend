use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;
use rand::Rng;
use rand::distributions::Alphanumeric;
use std::string::String;

fn generate_random_string(length: usize) -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    s
}

#[tokio::main]
pub async fn register_user(username: &str, password: &str, email: &str) -> Result<String, sqlx::Error> {
    dotenv().ok();
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&DATABASE_URL).await?;
    
    let token = generate_random_string(32);
    
    let row = sqlx::query("INSERT INTO user (username, password, email, token) VALUES (?, ?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(email)
        .bind(token.clone()) // 克隆 token
        .execute(&pool).await?;
    
    Ok(token)
}