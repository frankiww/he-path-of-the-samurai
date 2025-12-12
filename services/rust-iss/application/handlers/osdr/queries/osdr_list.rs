use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;
use sqlx::Row;
use crate::application::services::dtos::osdr_list_dto::{OsdrListDto, OsdrListResponse};
use crate::Core::GenericResponse;

pub async fn osdr_list(State(st): State<AppState>)
                       -> GenericResponse<OsdrListResponse> {
    let limit = std::env::var("OSDR_LIST_LIMIT").ok()
        .and_then(|s| s.parse::<i64>().ok()).unwrap_or(20);

    let rows = sqlx::query(
        "SELECT id, dataset_id, title, status, updated_at, inserted_at, raw
         FROM osdr_items
         ORDER BY inserted_at DESC
         LIMIT $1"
    ).bind(limit).fetch_all(&st.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let out: Vec<OsdrListDto> = rows.into_iter().map(|r| {
        OsdrListDto{
            id: r.get::<i64,_>("id"),
            dataset_id: r.get::<Option<String>,_>("dataset_id"),
            title: r.get::<Option<String>,_>("title"),
            status: r.get::<Option<String>,_>("status"),
            updated_at: r.get::<Option<DateTime<Utc>>,_>("updated_at"),
            inserted_at: r.get::<DateTime<Utc>, _>("inserted_at"),
            raw: r.get::<Value,_>("raw"),
        }
    }).collect();

    Ok(Json(OsdrListResponse{ items: out }))
}
