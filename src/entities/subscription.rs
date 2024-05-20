use chrono::{DateTime, Utc};
use sqlx::{postgres::PgQueryResult, PgPool};
use uuid::Uuid;

pub struct Subscription {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Utc>,
}

pub struct CreateSubscriptionInput {
    pub email: String,
    pub name: String,
}

impl Subscription {
    #[tracing::instrument(
        name = "Saving new subscriber details in the database", skip(input, pool),
        fields(
        subscriber_email = %input.email, subscriber_name = %input.name
    ))]
    pub async fn create(
        input: CreateSubscriptionInput,
        pool: PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        match sqlx::query!(
            r#"
            insert into subscriptions (id, email, name, subscribed_at)
            values ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            input.email,
            input.name,
            Utc::now()
        )
        .execute(&pool)
        .await
        {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!("Failed to execute query: {:?}", e);
                Err(e)
            }
        }
    }
}
