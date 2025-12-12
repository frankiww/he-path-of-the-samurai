use axum::{Json, Router};
use axum::routing::get;
use chrono::Utc;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use rust_iss::infrastructure::db::appstate::AppState;
use rust_iss::infrastructure::clients::iss;
use rust_iss::application::services::dtos::health_dto::HealthDto;
use rust_iss::api::routers::{space_router,health_router, iss_router, osdr_router};
use rust_iss::migrations::init::init_db;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);
    let state = AppState::new().await?;

    let app = Router::new()
        // общее
        .nest("", health_router::router())
        .with_state(state.clone())
        // ISS
        .nest("", iss_router::router())
        // OSDR
        .nest("", osdr_router::router())
        // Space cache
        .nest("", space_router::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", 3000)).await?;
    info!("rust_iss listening on 0.0.0.0:3000");
    let state = AppState::new().await?;
    init_db(&state.pool).await?;
    
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}