use crate::media::app_data::media_service::MediaServiceType;
use mediasoup::producer::ProducerId;

pub struct ConsumeArgsDto {
    pub producer_id: ProducerId,
    pub user_id: usize,
    pub service_type: MediaServiceType,
}
