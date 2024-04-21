use actix_web::{web, HttpRequest, Responder, HttpResponse, get};
use serde::Deserialize;
use serde_json::json;

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
