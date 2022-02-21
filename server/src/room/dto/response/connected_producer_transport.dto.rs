use actix::prelude::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct ConnectedProducerTransportResponse {}
