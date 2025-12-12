use std::time::Duration;
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_spacex_next(st: &AppState) -> anyhow::Result<Value> {
    let url = "https://api.spacexdata.com/v4/launches/next";
    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
    let json: Value = client.get(url).send().await?.json().await?;
    Ok(json)
}
