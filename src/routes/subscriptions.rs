use super::super::entities::subscription::{CreateSubscriptionInput, Subscription};
use axum::{extract::State, http::StatusCode, Form};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[axum::debug_handler]
pub async fn subscribe(State(pool): State<PgPool>, Form(data): Form<FormData>) -> StatusCode {
    match Subscription::create(
        CreateSubscriptionInput {
            email: data.email,
            name: data.name,
        },
        pool,
    )
    .await
    {
        Ok(_result) => StatusCode::OK,
        Err(_error) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
