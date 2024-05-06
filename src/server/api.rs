use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreateKVMServer {
    pub token: String,
    pub server_id: i32,
}

#[post("/api/server/kvm/create")]
pub async fn create_server_kvm(form: web::Form<CreateKVMServer>) -> impl Responder {

}