use actix::*;
use mediasoup::producer::ProducerId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ProducerPauseRequest {
    pub id: ProducerId,
    pub user_id: usize,
}
