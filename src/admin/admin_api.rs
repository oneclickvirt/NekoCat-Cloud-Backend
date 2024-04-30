use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use crate::admin::{login, user_list};

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct PageForm {
    pub token: String,
    pub page: i32,
}

#[post("/api/admin/login")]
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

#[post("/api/admin/users")]
pub async fn get_user_list(form: web::Form<PageForm>) -> impl Responder {
    let token = &form.token;
    let page = &form.page;
    
    let user_list = user_list::user_list(token, *page).await;

    match user_list {
        Ok(user_list) => {
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Get user list success",
                "data": user_list
            }))
        },
        Err(_) => {
            HttpResponse::Ok().json(json!({
                "status": "error",
                "message": "Get user list failed"
            }))
        }
    }
}