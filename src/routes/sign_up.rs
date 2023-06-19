use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignUpParams {
    display_name: String,
    email: String,
}

#[post("/sign_up")]
pub async fn sign_up(json: web::Json<SignUpParams>) -> HttpResponse {
    println!(
        "{} with email {} tried loggin in!",
        json.display_name, json.email
    );

    HttpResponse::Ok().finish()
}
