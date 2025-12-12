use axum::http::StatusCode;
use axum::Json;
// общий тип ответа для эндпоинтов
pub type GenericResponse<T> = Result<Json<T>, (StatusCode, String)>;