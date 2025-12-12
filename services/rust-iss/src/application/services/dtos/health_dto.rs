//data transfer object нужны чтобы типизировать 
//передаваемые данные между сервисами и обарботчиками
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthDto { pub status: &'static str, pub now: DateTime<Utc> }
