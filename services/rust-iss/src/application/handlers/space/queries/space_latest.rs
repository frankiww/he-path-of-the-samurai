use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;
use sqlx::Row;
use crate::application::services::dtos::space_latest_dto::{SpaceLatestRequestDto, SpaceLatestResponseDto, SpaceLatestResponseEmptyDto};
use crate::Core::GenericResponse;
use crate::infrastructure::db::repo::space_latest_repo;

pub async fn space_latest(Path(src): Path<String>, State(st): State<AppState>)
                          -> GenericResponse<SpaceLatestResponseDto> {
    let row = space_latest_repo::space_latest(&st.pool, &src.as_str()).await?;
    if let Some(r) = row {
        let fetched_at: DateTime<Utc> = r.get("fetched_at");
        let payload: Value = r.get("payload");
        return Ok(Json(SpaceLatestResponseDto::Res(SpaceLatestRequestDto { source: src, fetched_at: fetched_at, payload: payload })));
    }
    Ok(Json(SpaceLatestResponseDto::Empty(SpaceLatestResponseEmptyDto { source: src, message: "no data".to_owned() })))
}
