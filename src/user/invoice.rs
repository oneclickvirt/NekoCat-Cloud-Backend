// use sqlx::Row;
// use sqlx::mysql::MySqlPoolOptions;
// use std::env;
// use dotenv::dotenv;

// pub async fn get_invoice(invoice_id: i32) -> Result<String, sqlx::Error> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = MySqlPoolOptions::new().connect(&database_url).await?;
    
//     sqlx::query_as!("SELECT * FROM cart WHERE uuid = ?")
//         .bind(invoice_id)
//         .fetch_one(&pool).await?;
    
// }


// CREATE TABLE invoice (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `payment_id` INT NOT NULL DEFAULT 0,
//     `user_id` INT NOT NULL DEFAULT 0,
//     `invoice_remote` INT NOT NULL DEFAULT 0,
//     `invoice_local` INT NOT NULL DEFAULT 0,
//     `money` DECIMAL(10,2) NOT NULL DEFAULT 0.00,
//     `pay_status` INT NOT NULL DEFAULT 0, /*1: Paid 2: Unpaid 3: Expired 4: Fraud 5: Cancelled*/
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
//     `end_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );