use crate::routes::{health_check, sign_up};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, std::io::Error> {
    // Shadow the connection to a smart pointer, to allow app_data cloning.
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_pool.clone())
            .service(health_check)
            .service(sign_up)
    })
    .listen(listener)
    .unwrap()
    .run();

    Ok(server)
}
