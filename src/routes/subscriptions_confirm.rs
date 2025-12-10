use actix_web::{HttpResponse, ResponseError, web};
use anyhow::Context;
use reqwest::StatusCode;
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::error_chain_fmt;

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

#[tracing::instrument(name = "Confirm a pending subscriber", skip(parameters, pool))]
pub async fn confirm(
    parameters: web::Query<Parameters>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, SubscribeConfirmError> {
    let subscriber_id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Failed to retrieve the subscriber id associated with the provided token.")?
        .ok_or(SubscribeConfirmError::UknowTokenError)?;

    confirm_subscriber(&pool, subscriber_id)
        .await
        .context("Failed to update the subscriber status to `confirmed`.")?;

    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(subscriber_id, pool))]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tracing::instrument(name = "Get subscriber_id from token", skip(pool, subscription_token))]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let subscriber_id = sqlx::query!(
        r#"SELECT subscriber_id FROM subscription_tokens WHERE subscription_token = $1"#,
        subscription_token
    )
    .fetch_optional(pool)
    .await?
    .map(|row| row.subscriber_id);

    Ok(subscriber_id)
}

#[derive(thiserror::Error)]
pub enum SubscribeConfirmError {
    #[error("There is no subscriber associated with the provided token.")]
    UknowTokenError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for SubscribeConfirmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl ResponseError for SubscribeConfirmError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            SubscribeConfirmError::UknowTokenError => StatusCode::UNAUTHORIZED,
            SubscribeConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
