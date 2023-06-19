use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    // HttpResponseBuilder::new(StatusCode::OK)
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check))
        .listen(listener)
        .unwrap()
        .run();

    Ok(server)
}
