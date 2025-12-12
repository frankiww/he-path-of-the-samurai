use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::get;
use serde_json::Value;
use crate::application::handlers::space::commands::space_refresh::space_refresh;
use crate::application::handlers::space::queries::space_latest::space_latest;
use crate::application::handlers::space::queries::space_summary::space_summary;
use crate::application::services::dtos::refresh_result_dto::RefreshResultDto;
use crate::application::services::dtos::space_latest_dto::SpaceLatestResponseDto;
use crate::Core::GenericResponse;
use crate::infrastructure::db::appstate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/space/:src/latest", get(get_space_latest))
        .route("/space/refresh", get(get_space_refresh))
        .route("/space/summary", get(get_space_summary))
}

async fn get_space_latest(Path(src): Path<String>, State(st): State<AppState>) -> GenericResponse<SpaceLatestResponseDto> {
    space_latest(Path(src), State(st)).await
}

async fn get_space_refresh(Query(q): Query<HashMap<String, String>>,State(st): State<AppState>) -> GenericResponse<RefreshResultDto> {
    space_refresh(Query(q), State(st)).await
}

async fn get_space_summary(State(st): State<AppState>) -> Result<Json<Value>, (StatusCode, String)>  {
    space_summary(State(st)).await
}
        