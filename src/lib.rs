pub mod configuration;
pub mod routes;
pub mod startup;

// TODO(calco): Figure out what should be public and what should be private.
use crate::configuration::get_config;
use sqlx::PgPool;
use std::net::TcpListener;

pub use crate::startup::run;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Error binding to random port.");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("Failed to get config: ");
    let connection_pool =
        PgPool::connect(&config.db_settings.get_connection_string())
            .await
            .expect("Failed to connect to Postgres DB: ");

    let server =
        run(listener, connection_pool).expect("Failed to bind address.");

    // NOTES(calco): Non binding let, as we specifically DO NOT want to wait for completion.
    // (it never finishes)
    let sp = tokio::spawn(server);
    drop(sp);

    format!("127.0.0.1:{}", port)
}
