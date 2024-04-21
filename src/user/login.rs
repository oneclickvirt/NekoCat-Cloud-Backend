use actix_web::{HttpResponse, web, HttpRequest, get};
use actix_session::Session;
use handlebars::Handlebars;
use chrono::NaiveDateTime;
use serde_json::json;

#[get("/api/login")]
pub async fn user_login_get_token(username: String, password: String) -> Result<()> {
    
    Ok(1)
}