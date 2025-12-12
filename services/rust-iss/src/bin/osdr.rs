use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::osdr_service::fetch_and_store_osdr;

// фон OSDR
#[tokio::main]
async fn main() {
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_and_store_osdr(&st).await { error!("osdr err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_osdr)).await;
        }
    });
}