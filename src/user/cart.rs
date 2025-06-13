use crate::models::CartInfo;
use crate::error::ApiError;
use crate::db;

pub async fn get_cart(group_id: i32) -> Result<CartInfo, ApiError> {

    let pool = db::get_db_pool()?;    
    let cart_info = sqlx::query_as!
        (CartInfo,
        "SELECT * FROM cart WHERE group_id = ?",
        group_id)
        .fetch_one(pool)
        .await?;

    Ok(cart_info)
}