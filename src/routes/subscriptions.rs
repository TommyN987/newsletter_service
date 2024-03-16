use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use serde::Deserialize;
use sqlx::{types::chrono::Utc, PgPool};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, db_pool)
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: Form<FormData>, db_pool: Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&form, &db_pool).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, db_pool)
)]
pub async fn insert_subscriber(form: &FormData, db_pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
