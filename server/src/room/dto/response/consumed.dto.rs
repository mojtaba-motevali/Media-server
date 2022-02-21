use crate::media::app_data::media_service::MediaAppData;
use actix::prelude::Message;
use mediasoup::consumer::ConsumerId;
use mediasoup::producer::ProducerId;
use mediasoup::rtp_parameters::{MediaKind, RtpParameters};
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct ConsumedResponse {
    pub id: ConsumerId,
    pub producer_id: ProducerId,
    pub kind: MediaKind,
    pub rtp_parameters: RtpParameters,
    pub user_id: usize,
    pub app_data: MediaAppData,
}
