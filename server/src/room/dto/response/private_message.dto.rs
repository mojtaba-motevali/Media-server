use actix::prelude::Message;
use serde::Serialize;

use chrono::{DateTime, Utc};

#[derive(Serialize, Message, Debug)]
#[rtype(result = "()")]
pub struct PrivateMessageResponse {
    pub id: usize,
    pub message: String,
    pub receiver_id: usize,
    pub sender_id: usize,
    pub timestamp: DateTime<Utc>,
}
