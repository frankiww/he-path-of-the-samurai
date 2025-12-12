use std::time::Duration;
use chrono::Utc;
use serde_json::Value;
use crate::application::services::cache::write_cache;
use crate::infrastructure::clients::neows;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_neo_feed(st: &AppState) -> anyhow::Result<()> {
    let json: Value = neows::fetch_neo_feed(st).await?;
    write_cache(&st.pool, "neo", json).await
}