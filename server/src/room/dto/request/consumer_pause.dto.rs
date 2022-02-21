use actix::*;
use mediasoup::consumer::ConsumerId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ConsumerPauseRequest {
    pub user_id: usize,
    pub id: ConsumerId,
}
