use actix_web::{web, HttpRequest, Responder, HttpResponse, get};
use serde::Deserialize;
use serde_json::json;
use crate::user::login;

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

#[get("/status")]
pub async fn web_status() -> impl Responder {
    // 返回成功响应
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Service is running"
    }))
}


#[get("/api/user/login")]
pub async fn web_login(form: web::Query<LoginForm>) -> impl Responder {
    // 获取表单数据
    let username = &form.username;
    let password = &form.password;
    
    // 调用登录函数
    let token = login::user_login_get_token(username, password);
    
    // 返回成功响应
    match token {
        Ok(token) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Login success",
                "token": token
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