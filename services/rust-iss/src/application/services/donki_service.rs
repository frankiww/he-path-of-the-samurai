use chrono::Utc;
use serde_json::Value;
use crate::application::services::cache::write_cache;
use crate::infrastructure::clients::donki;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_donki(st: &AppState) -> anyhow::Result<()> {
    let _ = fetch_donki_flr(st).await;
    let _ = fetch_donki_cme(st).await;
    Ok(())
}

pub async fn fetch_donki_flr(st: &AppState) -> anyhow::Result<()> {
    let json: Value = donki::fetch_donki_flr(st).await?;
    write_cache(&st.pool, "flr", json).await
}
pub async fn fetch_donki_cme(st: &AppState) -> anyhow::Result<()> {
    let json: Value = donki::fetch_donki_cme(st).await?;
    write_cache(&st.pool, "cme", json).await
}

pub(crate) fn last_days(n: i64) -> (String, String) {
    let to = Utc::now().date_naive();
    let from = to - chrono::Days::new(n as u64);
    (from.to_string(), to.to_string())
}
