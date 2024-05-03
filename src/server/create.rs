use actix_web::Result;
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use sqlx::query_as;
use dotenv::dotenv;
use crate::models::ServerInfo;
use crate::server::proxmox;

pub async fn create_server(group_id: i32, cart_id: i32) -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new().connect(&database_url).await?;

    let choose_server: ServerInfo = query_as!(
        ServerInfo,
        "SELECT * FROM server WHERE `group` = ? ORDER BY RAND() LIMIT 1;",
        group_id
    )
    .fetch_one(&pool)
    .await?; // 使用 ? 处理可能的错误


    if choose_server.r#type == "1" {
        proxmox::create::pve_create_kvm(
            &choose_server.ip,
            &choose_server.port.to_string(),
            &choose_server.key,
            &choose_server.storage,
            &choose_server.node_name,
            &choose_server.network_name,
            "local:vztmpl/debian-10-standard_10.10-1_amd64.tar.gz",
            "test",
            "test",
        ).await?;
    }
Ok(())
}

// CREATE TABLE server (
//     `id` INT PRIMARY KEY AUTO_INCREMENT,
//     `name` VARCHAR(255) NOT NULL,
//     `ip` VARCHAR(255) NOT NULL,
//     `port` INT NOT NULL DEFAULT 0,
//     `type` VARCHAR(255) NOT NULL,
//     `max_limit` INT NOT NULL DEFAULT 0,
//     `group` INT NOT NULL DEFAULT 0,
//     `storage` VARCHAR(255) NOT NULL,
//     `node_name` VARCHAR(255) NOT NULL,
//     `network_name` VARCHAR(255) NOT NULL,
//     `is_nat` INT NOT NULL DEFAULT 0,
//     `nat_ip` VARCHAR(255) NOT NULL,
//     `status` INT NOT NULL DEFAULT 0,
//     `created_time` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
//   );