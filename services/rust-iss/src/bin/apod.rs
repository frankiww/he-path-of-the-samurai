use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::apod_service::fetch_apod;

// фон APOD
#[tokio::main]
async fn main() {
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_apod(&st).await { error!("apod err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_apod)).await;
        }
    });
}