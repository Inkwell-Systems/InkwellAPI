pub mod configuration;
pub mod routes;
pub mod startup;

use crate::routes::{health_check, sign_up};
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server =
        HttpServer::new(|| App::new().service(health_check).service(sign_up))
            .listen(listener)
            .unwrap()
            .run();

    Ok(server)
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error binding to random port.");
    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Failed to bind address.");

    // NOTES(calco): Non binding let, as we specifically DO NOT want to wait for completion.
    // (it never finishes)
    let sp = tokio::spawn(server);
    drop(sp);

    format!("127.0.0.1:{}", port)
}
