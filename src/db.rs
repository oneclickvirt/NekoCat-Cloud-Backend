use std::env;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use once_cell::sync::OnceCell;

use crate::error::ApiError;

static DB_POOL: OnceCell<Pool<MySql>> = OnceCell::new();

/// 初始化数据库连接池，只调用一次
pub async fn init_db_pool() -> Result<(), ApiError> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|e| ApiError::BadRequest(format!("DATABASE_URL error: {}", e)))?;

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .map_err(ApiError::Database)?;

    DB_POOL.set(pool).map_err(|_| ApiError::Other("DB_POOL already set".into()))?;

    Ok(())
}

/// 获取连接池（已初始化）
pub fn get_db_pool() -> Result<&'static Pool<MySql>, ApiError> {
    DB_POOL.get().ok_or_else(|| ApiError::Other("DB_POOL not initialized".into()))
}
