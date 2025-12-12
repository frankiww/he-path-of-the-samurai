use axum::extract::State;
use axum::{Router};
use axum::routing::get;
use crate::application::handlers::iss::commands::trigger_iss::trigger_iss;
use crate::application::handlers::iss::queries::get_iss_trend::iss_trend;
use crate::infrastructure::db::appstate::AppState;
use crate::application::handlers::iss::queries::get_last_iss::last_iss;
use crate::application::services::dtos::iss_fetch_log_dto::IssFetchLogEnumDto;
use crate::application::services::dtos::trend_dto::TrendDto;
use crate::Core::GenericResponse;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/last", get(get_last_iss))
        .route("/fetch", get(post_trigger_iss))
        .route("/iss/trend", get(get_iss_trend))
}


async fn get_last_iss(State(st): State<AppState>) -> GenericResponse<IssFetchLogEnumDto> {
    last_iss(State(st)).await
}

async fn post_trigger_iss(State(st): State<AppState>) -> GenericResponse<IssFetchLogEnumDto> {
    trigger_iss(State(st)).await
}

async fn get_iss_trend(State(st): State<AppState>) -> GenericResponse<TrendDto>  {
    iss_trend(State(st)).await
}