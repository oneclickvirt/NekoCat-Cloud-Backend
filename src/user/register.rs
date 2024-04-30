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

pub async fn register_user(username: &str, password: &str, email: &str, ip: &str) -> Result<String, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    
    let token = generate_random_string(32);
    
    let _row = sqlx::query("INSERT INTO users (username, password, email, token, last_login_ip) VALUES (?, ?, ?, ?, ?)")
        .bind(username)
        .bind(password)
        .bind(email)
        .bind(token.clone()) // 克隆 token
        .bind(ip)
        .execute(&pool).await?;
    
    Ok(token)
}