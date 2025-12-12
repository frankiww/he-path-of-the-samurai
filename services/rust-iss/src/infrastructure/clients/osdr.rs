use std::time::Duration;
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_osdr(st: &AppState) -> anyhow::Result<Value> {
    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
    let resp = client.get(&st.nasa_url).send().await?;
    if !resp.status().is_success() {
        anyhow::bail!("OSDR request status {}", resp.status());
    }
    let json: Value = resp.json().await?;
    Ok(json)
}