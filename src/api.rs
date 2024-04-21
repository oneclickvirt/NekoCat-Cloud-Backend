use actix_web::{web, HttpRequest, Responder, HttpResponse};
use serde::Deserialize;
use serde_json::json;
mod login;


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
    const MESSAGE: &str = "ilolicon IDC API Online";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    HttpResponse::Ok().json(json_response)
}

pub async fn user_login(form: web::Form<LoginForm>) -> impl Responder {
    // 调用登录函数进行用户验证并获取令牌
    if let Some(token) = crate::user::login::user_login_get_token(&form.username, &form.password) {
        // 如果验证成功，返回带有令牌的成功响应
        let json_response = json!({
            "status": "success",
            "message": "Login successful",
            "token": token
        });

        HttpResponse::Ok().json(json_response)
    } else {
        // 如果验证失败，返回相应的错误状态码和消息
        HttpResponse::BadRequest().json(json!({"status": "error", 
                    "message": "Invalid username or password"
            }))
    }
}

pub async fn register_user(req: HttpRequest, form: web::Form<RegisterFrom>) -> impl Responder {
    // 获取用户IP
    let user_ip = req.connection_info().peer_addr().unwrap_or("127.0.0.1");

    // 调用注册函数进行用户注册
    register::register_user(&form.username, &form.password, &form.email, &user_ip);

    // 返回成功响应
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "User registered successfully"
    }))
}