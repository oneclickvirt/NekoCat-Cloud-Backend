use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct UserInfo {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    pub money: Option<f64>,
    pub aff: Option<i32>,
    pub is_admin: Option<i32>,
    pub created_time: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<String>,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub token: Option<String>,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
pub struct UserRegister {
    pub id: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
    pub money: Option<f64>,
    pub aff: Option<i32>,
    pub is_admin: Option<i32>,
    pub created_time: Option<String>,
    pub last_login_ip: Option<String>,
    pub last_login_time: Option<String>,
}


#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Announcement {
    pub id: Option<i32>,
    pub announcement_title: Option<String>,
    pub announcement_body: Option<String>,
    pub display_portal: Option<i32>,
    pub created_time: Option<String>,
}
// CREATE TABLE announcement (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `type` INT NOT NULL DEFAULT 0, /*1: Normal 2: Emergency*/
//     `announcement_title` VARCHAR(255) NOT NULL,
//     `announcement_body` VARCHAR(255) NOT NULL,
//     `display_portal` INT NOT NULL DEFAULT 0,
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );