use crate::media::app_data::media_service::MediaServiceType;
use mediasoup::rtp_parameters::{MediaKind, RtpParameters};

pub struct ProduceArgsDto {
    pub kind: MediaKind,
    /// RTP parameters defining what the endpoint is sending.
    pub rtp_parameters: RtpParameters,
    pub user_id: usize,
    pub service_type: MediaServiceType,
}
