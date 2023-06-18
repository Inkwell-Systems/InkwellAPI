use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    // HttpResponseBuilder::new(StatusCode::OK)
    HttpResponse::Ok()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .bind("127.0.0.1:8000")
        .unwrap()
        .run();

    Ok(server)
}
