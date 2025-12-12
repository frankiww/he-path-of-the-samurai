use chrono::{DateTime, Utc};
use serde_json::Value;
//структура для записи о фетче iss
pub struct iss_fetch_log {
    pub id: i64,
    pub fetched_at: DateTime<Utc>,
    pub source_url: String,
    pub payload: Value,
}
