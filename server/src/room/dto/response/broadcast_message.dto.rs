use actix::prelude::Message;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Serialize)]
pub struct BroadcastMessageResponse {
    pub id: usize,
    pub sender_id: usize,
    pub user_name: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}
