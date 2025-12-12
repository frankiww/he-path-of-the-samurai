use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OsdrSyncDto {
    pub(crate) written: usize
}