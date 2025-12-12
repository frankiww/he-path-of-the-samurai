use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::neows_service::fetch_neo_feed;

#[tokio::main]
async fn main() // фон NeoWs
{
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_neo_feed(&st).await { error!("neo err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_neo)).await;
        }
    });
}