use actix::prelude::Message;
use mediasoup::producer::ProducerId;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct VideoProducedResponse {
    pub id: ProducerId,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct AudioProducedResponse {
    pub id: ProducerId,
}
