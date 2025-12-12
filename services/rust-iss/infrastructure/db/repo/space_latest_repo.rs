use axum::extract::State;
use axum::http::StatusCode;
use serde_json::Value;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::Row;
use crate::infrastructure::db::appstate::AppState;

pub async fn space_latest(pool: &PgPool, src: &str) -> anyhow::Result<Option<PgRow>, (StatusCode, String)> {
    sqlx::query(
        "SELECT fetched_at, payload FROM space_cache
         WHERE source = $1 ORDER BY id DESC LIMIT 1"
    ).bind(&src).fetch_optional(pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}