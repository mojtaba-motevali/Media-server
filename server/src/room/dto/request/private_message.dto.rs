use actix::prelude::Message;
use serde::Deserialize;

use chrono::{DateTime, Utc};

#[derive(Deserialize, Message, Debug)]
#[rtype(result = "()")]
pub struct PrivateMessageRequest {
    pub message: String,
    pub receiver_id: usize,
    pub sender_id: usize,
    pub timestamp: DateTime<Utc>,
}
