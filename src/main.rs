use inkwell_api::configuration::get_config;
use inkwell_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_config().expect("Failed to read configuration");
    
    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, config.application_port);
    let listener = TcpListener::bind(addr).unwrap();
    
    let server = run(listener).unwrap();
    server.await
}
