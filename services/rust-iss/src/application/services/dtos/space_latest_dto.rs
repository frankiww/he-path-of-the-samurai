use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct SpaceLatestRequestDto {
    pub source: String,
    pub fetched_at: DateTime<Utc>,
    pub payload: Value,
}

#[derive(Serialize, Deserialize)]
pub struct SpaceLatestResponseEmptyDto {
    pub source: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub enum SpaceLatestResponseDto {
    Res(SpaceLatestRequestDto),
    Empty(SpaceLatestResponseEmptyDto),
}