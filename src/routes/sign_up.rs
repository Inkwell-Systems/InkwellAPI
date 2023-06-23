use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct SignUpParams {
    display_name: String,
    email: String,
    profile_url: String,
}

#[post("/sign_up")]
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(json, connection_pool),
    fields(
        subscriber_email = %json.email,
        subscriber_name = %json.display_name
    )
)]
pub async fn sign_up(
    json: web::Json<SignUpParams>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    if !is_valid_name(&json.display_name) {
        return HttpResponse::BadRequest().finish();
    }

    let result = add_user_to_db(&json, &connection_pool).await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            tracing::error!("Error saving user to database: {:?}.", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub fn is_valid_name(name: &str) -> bool {
    let forbidden_chars = "/(){}[]|\"\\<>\'";

    let whitespace_check = name.trim().is_empty();
    let graphesme_check = name.graphemes(true).count() > 256;
    let forbidden_chars_check =
        name.chars().any(|c| forbidden_chars.contains(c));

    let f = whitespace_check || graphesme_check || forbidden_chars_check;
    !f
}

#[tracing::instrument(
    name = "Saving user details to database.",
    skip(json, connection_pool)
)]
async fn add_user_to_db(
    json: &web::Json<SignUpParams>,
    connection_pool: &web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    let uid = Uuid::new_v4();
    let created_at = Utc::now().date_naive();

    sqlx::query!(
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
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
