use std::collections::HashMap;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use redis::RedisResult;
use serde_json::Value;
use crate::infrastructure::db::appstate::AppState;
use sqlx::Row;
use tokio::io::AsyncBufReadExt;
use crate::application::services::apod_service::fetch_apod;
use crate::application::services::donki_service::{fetch_donki_cme, fetch_donki_flr};
use crate::application::services::dtos::refresh_result_dto::RefreshResultDto;
use crate::application::services::neows_service::fetch_neo_feed;
use crate::application::services::spacex_service::fetch_spacex_next;
use crate::Core::GenericResponse;

pub async fn space_refresh(Query(q): Query<HashMap<String, String>>, State(st): State<AppState>)
                           -> GenericResponse<RefreshResultDto> {
    let list = q.get("src").cloned().unwrap_or_else(|| "apod,neo,flr,cme,spacex".to_string());
    let mut done = Vec::new();
    for s in list.split(',').map(|x| x.trim().to_lowercase()) {
        match s.as_str() {
            "apod" => {
                let _ = fetch_apod(&st).await;
                done.push("apod".to_string());
            }
            "neo" => {
                let _ = fetch_neo_feed(&st).await;
                done.push("neo".to_string());
            }
            "flr" => {
                let _ = fetch_donki_flr(&st).await;
                done.push("flr".to_string());
            }
            "cme" => {
                let _ = fetch_donki_cme(&st).await;
                done.push("cme".to_string());
            }
            "spacex" => {
                let _ = fetch_spacex_next(&st).await;
                done.push("spacex".to_string());
            }
            _ => {}
        }
    }
    Ok(Json(RefreshResultDto{ refreshed: done }))
}
