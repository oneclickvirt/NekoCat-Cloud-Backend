use sqlx::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
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
// CREATE TABLE announcement (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `type` INT NOT NULL DEFAULT 0, /*1: Normal 2: Emergency*/
//     `announcement_title` VARCHAR(255) NOT NULL,
//     `announcement_body` VARCHAR(255) NOT NULL,
//     `display_portal` INT NOT NULL DEFAULT 0,
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct ServerInfo {
    pub id: i32,
    pub name: String,
    pub ip: String,
    pub port: i32,
    pub r#type: String,
    pub max_limit: i32,
    pub group: i32,
    pub status: i32,
    pub created_time: NaiveDateTime,
}
// CREATE TABLE server (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `name` VARCHAR(255) NOT NULL,
//     `ip` VARCHAR(255) NOT NULL,
//     `port` INT NOT NULL DEFAULT 0,
//     `type` VARCHAR(255) NOT NULL,
//     `max_limit` INT NOT NULL DEFAULT 0,
//     `group` INT NOT NULL DEFAULT 0,
//     `status` INT NOT NULL DEFAULT 0,
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );