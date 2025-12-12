use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use crate::application::handlers::iss::queries::get_last_iss::last_iss;
use crate::application::services::dtos::iss_fetch_log_dto::IssFetchLogEnumDto;
use crate::infrastructure::db::appstate::AppState;
use crate::application::services::iss_service::fetch_and_store_iss;
use crate::Core::GenericResponse;
//запускает обновление данных iss
pub async fn trigger_iss(State(st): State<AppState>)
                         -> GenericResponse<IssFetchLogEnumDto> {
    let fallback_url = st.fallback_url.as_str();
    fetch_and_store_iss(&st.pool, fallback_url).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    last_iss(State(st)).await
}