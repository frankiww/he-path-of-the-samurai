use chrono::{DateTime, Utc};
use serde_json::Value;
//структура для кэша
pub struct space_cache {
    pub id: i64,
    pub source: String,
    pub fetched_at: DateTime<Utc>,
    pub payload: Value,
}