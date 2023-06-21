use inkwell_api::configuration::get_config;
use inkwell_api::run;
use inkwell_api::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("inkwell".into(), "info".into());
    init_subscriber(subscriber);

    // Configure application.
    let config = get_config().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(
        config.db_settings.get_connection_string().expose_secret(),
    )
    .await
    .expect("Failed to connect to Postgres DB: ");

    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, config.application_port);
    let listener = TcpListener::bind(addr).unwrap();

    let server = run(listener, connection_pool).unwrap();
    server.await
}
