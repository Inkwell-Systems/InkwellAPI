use crate::domain::{DisplayName, Email, UserIncomplete};
use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// TODO(calco): Consider adding this to the domain?
#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpParams {
    pub display_name: String,
    pub email: String,
    pub profile_url: String,
}

// TODO(calco): Implement proper actix error handling.
// https://actix.rs/docs/errors/
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
    // Use json.0 due to ownership stuff.

    let user_i = match UserIncomplete::parse(
        json.0.email,
        json.0.display_name,
        json.0.profile_url,
    ) {
        Ok(user_i) => user_i,
        Err(err) => return HttpResponse::BadRequest().finish(),
    };

    match add_user_to_db(user_i, &connection_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            tracing::error!("Error saving user to database: {:?}.", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[tracing::instrument(
    name = "Saving user details to database.",
    skip(i_user, connection_pool)
)]
async fn add_user_to_db(
    i_user: UserIncomplete,
    connection_pool: &web::Data<PgPool>,
) -> Result<(), sqlx::Error> {
    let uid = Uuid::new_v4();
    let created_at = Utc::now().date_naive();

    sqlx::query!(
        r#"INSERT INTO users(uid, display_name, email, profile_url, created_at) 
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uid,
        i_user.display_name.as_ref(),
        i_user.email.as_ref(),
        i_user.profile_url,
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
