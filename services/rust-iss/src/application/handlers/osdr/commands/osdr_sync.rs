use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::Value;
use crate::application::services::dtos::osdr_sync_dto::OsdrSyncDto;
use crate::application::services::osdr_service::fetch_and_store_osdr;
use crate::infrastructure::db::appstate::AppState;


pub async fn osdr_sync(State(st): State<AppState>)
                       -> Result<Json<OsdrSyncDto>, (StatusCode, String)> {
    let written: usize = fetch_and_store_osdr(&st).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(OsdrSyncDto { written: written }))
}