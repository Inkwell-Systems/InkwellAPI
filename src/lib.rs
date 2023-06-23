pub mod configuration;
pub mod domain;
pub mod routes;
pub mod startup;

pub mod telemetry;

// TODO(calco): Figure out what should be public and what should be private.
pub use crate::startup::run;

use sqlx::PgPool;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}
