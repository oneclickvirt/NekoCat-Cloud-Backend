use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(FromRow, Deserialize, Serialize, sqlx::Type)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub token: String,
    pub money: f64,
    pub aff: i32,
    pub is_admin: i32,
    pub created_time: NaiveDateTime,
    pub last_login_ip: String,
    pub last_login_time: NaiveDateTime,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub token: String,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
pub struct UserRegister {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub token: String,
    pub money: f64,
    pub aff: i32,
    pub is_admin: i32,
    pub created_time: NaiveDateTime,
    pub last_login_ip: String,
    pub last_login_time: NaiveDateTime,
}


#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Announcement {
    pub id: i32,
    pub announcement_title: String,
    pub announcement_body: String,
    pub display_portal: i32,
    pub created_time: NaiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ServerInfo {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub key: String,
    pub r#type: String,
    pub max_limit: i32,
    pub group: i32,
    pub storage: String,
    pub node_name: String,
    pub network_name: String,
    pub vm_technology: i32,
    pub is_nat: i32,
    pub nat_ip: String,
    pub status: i32,
    pub created_time: NaiveDateTime,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct CartInfo {
    pub id: i32,
    pub group_id: i32,
    pub cart_title: String,
    pub cart_body: String,
    pub money: Decimal,
    pub stock: i32,
    pub allow_payment: String,
    pub created_time: NaiveDateTime,
}
