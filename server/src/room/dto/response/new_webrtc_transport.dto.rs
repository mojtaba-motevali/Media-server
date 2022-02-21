use crate::media::media_struct::TransportOptions;
use actix::prelude::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct NewWebrtcTransportResponse {
    pub transport_type: String,
    pub webrtc_transport: TransportOptions,
}
