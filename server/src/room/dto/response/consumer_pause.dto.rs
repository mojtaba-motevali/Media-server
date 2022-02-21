use actix::*;
use mediasoup::consumer::ConsumerId;
use serde::Serialize;

#[derive(Serialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ConsumerPauseResponse {
    pub user_id: usize,
    pub id: ConsumerId,
}
