use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::donki_service::fetch_donki;

#[tokio::main]
async fn main()
// фон DONKI
{
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_donki(&st).await { error!("donki err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_donki)).await;
        }
    });
}