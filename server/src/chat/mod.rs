use chrono::{DateTime, Utc};
use rand::{self, rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateMessage {
    pub id: usize,
    pub message: String,
    pub receiver_id: usize,
    pub sender_id: usize,
    pub timestamp: DateTime<Utc>,
}
impl PrivateMessage {
    pub fn new(
        sender_id: usize,
        receiver_id: usize,
        message: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        let mut rng = ThreadRng::default();
        Self {
            id: rng.gen::<usize>(),
            sender_id,
            message,
            receiver_id,
            timestamp,
        }
    }
}
impl Clone for PrivateMessage {
    fn clone(&self) -> PrivateMessage {
        PrivateMessage {
            id: self.id,
            message: self.message.clone(),
            receiver_id: self.receiver_id,
            sender_id: self.sender_id,
            timestamp: self.timestamp.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastMessage {
    pub id: usize,
    pub sender_id: usize,
    pub message: String,
    pub user_name: String,
    pub timestamp: DateTime<Utc>,
}
impl BroadcastMessage {
    pub fn new(
        sender_id: usize,
        message: String,
        user_name: String,
        timestamp: DateTime<Utc>,
    ) -> Self {
        let mut rng = ThreadRng::default();
        Self {
            id: rng.gen::<usize>(),
            sender_id,
            message,
            user_name,
            timestamp,
        }
    }
}
impl Clone for BroadcastMessage {
    fn clone(&self) -> BroadcastMessage {
        BroadcastMessage {
            id: self.id,
            sender_id: self.sender_id.clone(),
            message: self.message.clone(),
            user_name: self.user_name.clone(),
            timestamp: self.timestamp.clone(),
        }
    }
}
