use std::time::Duration;
use tracing::error;
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::application::services::spacex_service::fetch_spacex_next;

#[tokio::main]
async fn main()
// фон SpaceX
{
    let st = AppState::new().await.unwrap();
    tokio::spawn(async move {
        loop {
            if let Err(e) = fetch_spacex_next(&st).await { error!("spacex err {e:?}") }
            tokio::time::sleep(Duration::from_secs(st.every_spacex)).await;
        }
    });
}