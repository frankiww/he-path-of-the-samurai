use std::time::Duration;
use serde_json::Value;
use crate::application::services::donki_service::last_days;
use crate::infrastructure::db::appstate::AppState;

pub async fn fetch_donki_flr(st: &AppState) -> anyhow::Result<Value> {
    let (from, to) = crate::application::services::donki_service::last_days(5);
    let url = "https://api.nasa.gov/DONKI/FLR";
    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[("startDate", from), ("endDate", to)]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key", &st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    Ok(json)
}

pub async fn fetch_donki_cme(st: &AppState) -> anyhow::Result<Value> {
    let (from, to) = last_days(5);
    let url = "https://api.nasa.gov/DONKI/CME";
    let client = reqwest::Client::builder().timeout(Duration::from_secs(30)).build()?;
    let mut req = client.get(url).query(&[("startDate", from), ("endDate", to)]);
    if !st.nasa_key.is_empty() { req = req.query(&[("api_key", &st.nasa_key)]); }
    let json: Value = req.send().await?.json().await?;
    Ok(json)
}