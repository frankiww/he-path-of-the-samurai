use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub enum IssFetchLogEnumDto {
    Empty(IssFetchLogResponseEmptyDto),
    Res(IssFetchLogResponseDto),
}


#[derive(Serialize, Deserialize)]
pub struct IssFetchLogResponseDto {
    pub id: i64,
    pub fetched_at: DateTime<Utc>,
    pub source_url: String,
    pub payload: Value,
}

#[derive(Serialize, Deserialize)]
pub struct IssFetchLogResponseEmptyDto {
    pub message: String,
}