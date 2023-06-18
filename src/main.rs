use actix_web::http::StatusCode;
use actix_web::{
    get, App, HttpResponseBuilder, HttpServer, Responder,
};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run()
        .await
}
