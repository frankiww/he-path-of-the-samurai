use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;
use sqlx::Row;
use crate::application::services::dtos::iss_fetch_log_dto::{IssFetchLogEnumDto, IssFetchLogResponseDto, IssFetchLogResponseEmptyDto};
use crate::Core::GenericResponse;

pub async fn last_iss(State(st): State<AppState>)
                      -> GenericResponse<IssFetchLogEnumDto> {
    let row_opt = sqlx::query(
        "SELECT id, fetched_at, source_url, payload
         FROM iss_fetch_log
         ORDER BY id DESC LIMIT 1"
    ).fetch_optional(&st.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(row) = row_opt {
        let id: i64 = row.get("id");
        let fetched_at: DateTime<Utc> = row.get::<DateTime<Utc>, _>("fetched_at");
        let source_url: String = row.get("source_url");
        let payload: Value = row.try_get("payload").unwrap_or(serde_json::json!({}));
        return Ok(Json(IssFetchLogEnumDto::Res(IssFetchLogResponseDto {
            id: id,
            fetched_at: fetched_at,
            source_url: source_url,
            payload: payload,
        })));
    }
    Ok(Json(IssFetchLogEnumDto::Empty(IssFetchLogResponseEmptyDto { message: "no data".to_owned() })))
}