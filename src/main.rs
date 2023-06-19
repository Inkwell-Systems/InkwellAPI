use inkwell_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    let server = run(listener).unwrap();
    server.await
}
