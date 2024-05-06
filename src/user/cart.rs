use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;
use crate::models::CartInfo;
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub async fn get_cart(group_id: i32) -> Result<CartInfo, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new().connect(&database_url).await?;
    
    let mut cart_info = sqlx::query_as!
        (CartInfo,
        "SELECT * FROM cart WHERE group_id = ?",
        group_id)
        .fetch_one(&pool)
        .await?;

    // Convert the money field from String to BigDecimal
    cart_info.money = BigDecimal::from_str(&cart_info.money).unwrap();

    Ok(cart_info)
}

// CREATE TABLE cart (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `group_id` INT NOT NULL DEFAULT 0,
//     `cart_title` VARCHAR(255) NOT NULL,
//     `cart_body` VARCHAR(255) NOT NULL,
//     `money` DECIMAL(10,2) NOT NULL DEFAULT 0.00,
//     `stock` INT NOT NULL DEFAULT 0,
//     `allow_payment` VARCHAR(255) NOT NULL,
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );