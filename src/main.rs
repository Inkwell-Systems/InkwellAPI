use inkwell_api::configuration::get_config;
use inkwell_api::run;
use inkwell_api::telemetry::{get_subscriber, init_subscriber};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::info_span;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("inkwell".into(), "info".into());
    init_subscriber(subscriber);

    // Configure application.
    let config = get_config().expect("Failed to read configuration");
    let connection_pool = PgPool::connect_lazy(
        config.db_settings.get_connection_string().expose_secret(),
    )
    .expect("Failed to connect to Postgres DB: ");

    info_span!(
        "Starting server.",
        config.app_settings.address,
        config.app_settings.port
    );

    let addr = format!(
        "{}:{}",
        config.app_settings.address, config.app_settings.port
    );
    let listener = TcpListener::bind(addr).unwrap();

    let server = run(listener, connection_pool).unwrap();
    server.await
}
