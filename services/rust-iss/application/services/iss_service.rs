use std::time::Duration;
use serde_json::Value;
use sqlx::PgPool;
use crate::infrastructure::clients::iss;

pub async fn fetch_and_store_iss(pool: &PgPool, url: &str) -> anyhow::Result<()> {
    let json: Value = iss::fetch_iss(url).await?;
    sqlx::query("INSERT INTO iss_fetch_log (source_url, payload) VALUES ($1, $2)")
        .bind(url).bind(json).execute(pool).await?;
    Ok(())
}