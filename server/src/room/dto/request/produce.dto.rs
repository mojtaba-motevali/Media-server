use crate::media::app_data::media_service::MediaServiceType;
use actix::*;
use mediasoup::rtp_parameters::{MediaKind, RtpParameters};
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ProduceRequest {
    pub user_id: usize,
    pub transport_id: String,
    pub service_type: MediaServiceType,
    pub kind: MediaKind,
    pub rtp_parameters: RtpParameters,
}
