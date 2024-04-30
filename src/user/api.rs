use actix_web::{web, Responder, HttpResponse, get, post};
use serde::Deserialize;
use serde_json::json;
use crate::user::{login, register, announcement, cart};
#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterFrom {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct CartFrom {
    pub group_id: String,
}

#[get("/status")]
pub async fn web_status() -> impl Responder {
    // 返回成功响应
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Service is running"
    }))
}

#[post("/api/user/login")]
pub async fn web_login(form: web::Form<LoginForm>) -> impl Responder {
    // 获取表单数据
    let username = &form.username;
    let password = &form.password;
    
    let user_token = login::user_login_get_token(&username, &password).await;
    // 返回成功响应
    match user_token {
        Ok(user_token) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Login success",
                "token": user_token.token
            }))
        },
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "status": "error",
                "message": "Login failed"
            }))
        }
    }
}

#[post("/api/user/register")]
pub async fn web_register(form: web::Form<RegisterFrom>) -> impl Responder {
    // 获取表单数据
    let username = &form.username;
    let password = &form.password;
    let email = &form.email;
    
    let token = register::register_user(username, password, email).await;
    
    // 返回成功响应
    match token {
        Ok(token) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Register success",
                "token": token
            }))
        },
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "status": "error",
                "message": "Register failed"
            }))
        }
    }
}

#[get("/announcement")]
pub async fn web_announcement() -> impl Responder {
    let announcement_result = announcement::get_announcement().await;  // 确保异步调用被正确处理

    match announcement_result {
        Ok(announcement) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Get announcement success",
                "announcement": announcement
            }))
        },
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "status": "error",
                "message": "Get announcement failed"
            }))
        }
    }
}