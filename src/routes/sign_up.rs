use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{PgConnection, PgPool};
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
    // TODO(calco): Actually do stuff database.

    println!("{:?}", json);

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
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
