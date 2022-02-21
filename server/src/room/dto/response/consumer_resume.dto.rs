use actix::*;
use mediasoup::consumer::ConsumerId;
use serde::Serialize;

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct ConsumerResumeResponse {
    pub user_id: usize,
    pub id: ConsumerId,
}
