use axum::extract::State;
use axum::{Json, Router};
use axum::routing::get;
use crate::application::handlers::osdr::commands::osdr_sync::{osdr_sync};
use crate::application::handlers::osdr::queries::osdr_list::osdr_list;
use crate::application::services::dtos::osdr_list_dto::OsdrListResponse;
use crate::application::services::dtos::osdr_sync_dto::OsdrSyncDto;
use crate::Core::GenericResponse;
use crate::infrastructure::db::appstate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/osdr/sync", get(get_osdr_sync))
        .route("/osdr/list", get(get_osdr_list))
}

async fn get_osdr_sync(State(st): State<AppState>) -> GenericResponse<OsdrSyncDto> {
    osdr_sync(State(st)).await
}

async fn get_osdr_list(State(st): State<AppState>) -> GenericResponse<OsdrListResponse> {
    osdr_list(State(st)).await
}