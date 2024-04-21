use sqlx::mysql::mysqlPoolOptions;
mod config;
use config::DB_URL;

pub fn read_only(table: &str, column: &str, sqlwhere: &str) -> Vec<my::Row> {
    let pool = mysqlPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await
        .expect("Failed to create pool");

    let rec = sqlx::query!("SELECT {} FROM {} WHERE={}", "name", "users", "sqlwhere", 1)
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch all");

    rec
}

pub fn write_only(tables: &str, sqlcolumn: &str, data: &str) -> Vec<my::Row> {
    let pool = mysqlPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await
        .expect("Failed to create pool");

    let rec = sqlx::query!("INSERT INTO {} ({}) VALUES ({})", "users", "name", "data")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch all");

    rec
}