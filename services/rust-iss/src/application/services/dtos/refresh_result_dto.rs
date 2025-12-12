use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RefreshResultDto {
    pub refreshed: Vec<String>
}