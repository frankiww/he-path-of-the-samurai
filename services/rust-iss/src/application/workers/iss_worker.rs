use std::time::Duration;
use crate::infrastructure::db::appstate::AppState;
use crate::application::services::iss_service::fetch_and_store_iss;
use tracing::error;
use tracing::info;

pub async fn start_iss_worker(st: AppState) {
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_and_store_iss(&st.pool, &st.fallback_url).await {
                error!("iss worker error: {:?}", e);
            } else {
                info!("iss worker successfully fetched and stored data");
            }
            tokio::time::sleep(Duration::from_secs(st.every_iss)).await;
        }
    });
}
