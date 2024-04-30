use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

#[tokio::main]
pub async fn get_cart(group_id: i32) -> Result<String, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    
    let row = sqlx::query("SELECT * FROM cart WHERE group_id = ?")
        .bind(group_id)
        .fetch_one(&pool).await?;
    
    // 从数据库返回购物车
    let cart_title: String = row.try_get("cart_title")?;
    Ok(cart_title)
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