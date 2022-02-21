use actix::prelude::Message;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastMessageRequest {
    pub sender_id: usize,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}
