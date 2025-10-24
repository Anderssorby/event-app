use time::OffsetDateTime;
use serde::{Serialize, Deserialize}; 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewEvent {
    pub title: String,
    pub description: String,
    pub date_time: OffsetDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date_time: OffsetDateTime,
}
