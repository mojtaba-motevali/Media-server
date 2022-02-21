use actix::*;
use mediasoup::producer::ProducerId;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ProducerResumeRequest {
    pub user_id: usize,
    pub id: ProducerId,
}
