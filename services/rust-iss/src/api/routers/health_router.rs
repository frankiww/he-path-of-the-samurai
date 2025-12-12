use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use chrono::Utc;
use crate::infrastructure::db::appstate::AppState;
use crate::application::services::dtos::health_dto::HealthDto;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health(State(st): State<AppState>) -> Json<HealthDto> {
    Json(HealthDto { status: "ok", now: Utc::now() })
}