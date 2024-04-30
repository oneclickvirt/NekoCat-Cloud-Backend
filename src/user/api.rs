use actix_web::{web, HttpRequest, Responder, HttpResponse, get, post};
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
pub async fn web_login(form: web::Query<LoginForm>) -> impl Responder {
    // 获取表单数据
    let username = &form.username;
    let password = &form.password;
    
    let token = login::user_login_get_token(username, password);

        HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "Login success",
            "token": token
        }))
}

#[post("/api/user/register")]
pub async fn web_register(form: web::Query<RegisterFrom>) -> impl Responder {
    // 获取表单数据
    let username = &form.username;
    let password = &form.password;
    let email = &form.email;
    
    let token = register::register_user(username, password, email);
    
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
    // 获取公告
    let announcement = announcement::get_announcement();
    
    // 返回成功响应
    match announcement {
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