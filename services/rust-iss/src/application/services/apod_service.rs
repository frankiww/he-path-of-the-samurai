// каждый сервис отвечает за работу с конкретным источником данных
use serde_json::Value;
use crate::application::services::cache::write_cache;
use crate::infrastructure::db::appstate::AppState;
use crate::infrastructure::clients::apod;

pub async fn fetch_apod(st: &AppState) -> anyhow::Result<()> {
    let json: Value = apod::fetch_apod(st).await?;
    write_cache(&st.pool, "apod", json).await
}