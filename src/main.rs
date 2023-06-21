use inkwell_api::configuration::get_config;
use inkwell_api::run;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Configure telemetry tracing.
    LogTracer::init().expect("Failed to set log tracer.");

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer =
        BunyanFormattingLayer::new(String::from("inkwell"), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Failed to set subscriber.");

    // Configure application.
    let config = get_config().expect("Failed to read configuration");
    let connection_pool =
        PgPool::connect(&config.db_settings.get_connection_string())
            .await
            .expect("Failed to connect to Postgres DB: ");

    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, config.application_port);
    let listener = TcpListener::bind(addr).unwrap();

    let server = run(listener, connection_pool).unwrap();
    server.await
}
