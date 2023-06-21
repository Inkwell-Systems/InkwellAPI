use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SignUpParams {
    display_name: String,
    email: String,
    profile_url: String,
}

#[post("/sign_up")]
pub async fn sign_up(
    json: web::Json<SignUpParams>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    let req_uid = Uuid::new_v4();
    let req_span = tracing::info_span!(
        "Sign up request received for user",
        %req_uid,
        sub_email = %json.email,
        sub_name = %json.display_name
    );
    let query_span = tracing::info_span!("Attempting to add user to database.");

    let uid = Uuid::new_v4();
    let created_at = Utc::now().date_naive();
    let result = sqlx::query!(
        r#"INSERT INTO users(uid, display_name, email, profile_url, created_at) 
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uid,
        json.display_name,
        json.email,
        json.profile_url,
        created_at
    )
    .execute(connection_pool.get_ref())
    .instrument(query_span)
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            tracing::error!("Error saving user to database: {:?}.", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
