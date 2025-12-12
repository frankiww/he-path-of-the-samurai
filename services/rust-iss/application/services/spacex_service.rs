use std::time::Duration;
use serde_json::Value;
use crate::application::services::cache::write_cache;
use crate::infrastructure::clients::spacex;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_spacex_next(st: &AppState) -> anyhow::Result<()> {
    let json: Value = spacex::fetch_spacex_next(st).await?;
    write_cache(&st.pool, "spacex", json).await
}
