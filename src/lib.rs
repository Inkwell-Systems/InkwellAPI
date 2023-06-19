use actix_web::dev::Server;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    // HttpResponseBuilder::new(StatusCode::OK)
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct SignUpParams {
    display_name: String,
    email: String,
}

#[post("/sign_up")]
async fn sign_up(json: web::Json<SignUpParams>) -> HttpResponse {
    println!(
        "{} with email {} tried loggin in!",
        json.display_name, json.email
    );

    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(health_check).service(sign_up))
        .listen(listener)
        .unwrap()
        .run();

    Ok(server)
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Error binding to random port.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address.");

    // NOTES(calco): Non binding let, as we specifically DO NOT want to wait for completion.
    // (it never finishes)
    let sp = tokio::spawn(server);
    drop(sp);

    format!("127.0.0.1:{}", port)
}
