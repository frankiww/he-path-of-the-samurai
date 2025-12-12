use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::iss_service::fetch_and_store_iss;

// фон ISS
#[tokio::main]
async fn main() {
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_and_store_iss(&st.pool, &st.fallback_url.as_str()).await { error!("iss err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_iss)).await;
        }
    });
}