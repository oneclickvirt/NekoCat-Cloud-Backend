use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreateServer {
    pub token: String,
}

#[post("/api/server/create")]
pub async fn web_login(form: web::Form<CreateServer>) -> impl Responder {

}