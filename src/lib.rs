pub mod configuration;
pub mod routes;
pub mod startup;

pub mod telemetry;

// TODO(calco): Figure out what should be public and what should be private.
pub use crate::startup::run;

use crate::configuration::get_config;
use crate::telemetry::{get_subscriber, init_subscriber};
use once_cell::sync::Lazy;
use sqlx::{Executor, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("inkwell".into(), "info".into());
    init_subscriber(subscriber);
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error binding to random port.");
    let port = listener.local_addr().unwrap().port();

    let mut config = get_config().expect("Failed to get config: ");
    config.db_settings.db_name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&config.db_settings).await;

    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address.");

    // NOTES(calco): Non binding let, as we specifically DO NOT want to wait for completion.
    // (it never finishes)
    let sp = tokio::spawn(server);
    drop(sp);

    TestApp {
        address: format!("127.0.0.1:{}", port),
        db_pool: connection_pool,
    }
}

pub async fn configure_database(
    config: &configuration::DatabaseSettings,
) -> PgPool {
    // Create database
    let connection =
        PgPool::connect(&config.get_connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres.");

    connection
        .execute(&*format!("CREATE DATABASE \"{}\";", config.db_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}
