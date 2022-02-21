use actix::*;
use mediasoup::consumer::ConsumerId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ConsumerCloseRequest {
    pub user_id: usize,
    pub id: ConsumerId,
    pub producer_user_id: usize,
}
