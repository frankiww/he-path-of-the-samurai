use std::time::Duration;
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_iss(url: &str) -> anyhow::Result<Value> {
    let client = reqwest::Client::builder().timeout(Duration::from_secs(20)).build()?;
    let resp = client.get(url).send().await?;
    let json: Value = resp.json().await?;
    Ok(json)
}