use chrono::{DateTime, Utc};
use serde_json::Value;
//структура для osdr-данных
pub struct osdr_items {
    pub id: i64,
    pub dataset_id: String,
    pub title: String,
    pub status: String,
    pub updated_at: DateTime<Utc>,
    pub inserted_at: DateTime<Utc>,
    pub raw: Value,
}
