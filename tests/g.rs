use inkwell_api::configuration::get_config;
use inkwell_api::telemetry::{get_subscriber, init_subscriber};
use inkwell_api::{configuration, run, TestApp};
use once_cell::sync::Lazy;
use secrecy::{ExposeSecret, Secret};
use sqlx::{Executor, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

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
    config.db_settings.db_name = Secret::new(Uuid::new_v4().to_string());

    println!("DB NAME: {}", config.db_settings.db_name.expose_secret());

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
        PgPool::connect_lazy_with(config.get_connection_details_no_db());

    connection
        .execute(&*format!(
            "CREATE DATABASE \"{}\";",
            config.db_name.expose_secret()
        ))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool =
        PgPool::connect_lazy_with(config.get_connection_details());

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database.");

    connection_pool
}
