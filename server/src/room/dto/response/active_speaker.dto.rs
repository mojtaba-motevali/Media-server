use actix::prelude::Message;
use mediasoup::producer::ProducerId;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct ActiveSpeakerResponse {
    pub producer_id: ProducerId,
    pub user_id: usize,
    pub volume: i8,
}
