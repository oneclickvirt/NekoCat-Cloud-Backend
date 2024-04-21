use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use crate::config::DATABASE_URL;

#[tokio::main]
pub async fn get_cart(group_id: i32) -> Result<String, sqlx::Error> {
    let pool = MySqlPoolOptions::new().connect(DATABASE_URL).await?;
    
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