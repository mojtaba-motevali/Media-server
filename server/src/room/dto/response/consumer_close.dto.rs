use actix::*;
use mediasoup::consumer::ConsumerId;
use serde::Serialize;

#[derive(Debug, Message, Serialize)]
#[rtype(result = "()")]
pub struct ConsumerCloseResponse {
    pub user_id: usize,
    pub id: ConsumerId,
}
