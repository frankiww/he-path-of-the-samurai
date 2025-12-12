use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::application::services::cache::latest_from_cache;
use crate::infrastructure::db::appstate::AppState;
use sqlx::Row;


pub async fn space_summary(State(st): State<AppState>)
                           -> Result<Json<Value>, (StatusCode, String)> {
    let apod = latest_from_cache(&st.pool, "apod").await;
    let neo = latest_from_cache(&st.pool, "neo").await;
    let flr = latest_from_cache(&st.pool, "flr").await;
    let cme = latest_from_cache(&st.pool, "cme").await;
    let spacex = latest_from_cache(&st.pool, "spacex").await;

    let iss_last = sqlx::query("SELECT fetched_at,payload FROM iss_fetch_log ORDER BY id DESC LIMIT 1")
        .fetch_optional(&st.pool).await.ok().flatten()
        .map(|r| serde_json::json!({"at": r.get::<DateTime<Utc>,_>("fetched_at"), "payload": r.get::<Value,_>("payload")}))
        .unwrap_or(serde_json::json!({}));

    let osdr_count: i64 = sqlx::query("SELECT count(*) AS c FROM osdr_items")
        .fetch_one(&st.pool).await.map(|r| r.get::<i64, _>("c")).unwrap_or(0);

    Ok(Json(serde_json::json!({
        "apod": apod, "neo": neo, "flr": flr, "cme": cme, "spacex": spacex,
        "iss": iss_last, "osdr_count": osdr_count
    })))
}
