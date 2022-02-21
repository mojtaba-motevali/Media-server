use super::app_data::media_service::MediaAppData;
use actix::*;
use mediasoup::producer::ProducerId;
use serde::{Deserialize, Serialize};

use mediasoup::data_structures::{DtlsParameters, IceCandidate, IceParameters};
use mediasoup::transport::TransportId;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct DataProducer {
    pub id: ProducerId,
    pub app_data: MediaAppData,
    pub user_id: usize,
}
impl Clone for DataProducer {
    fn clone(&self) -> DataProducer {
        Self {
            id: self.id,
            app_data: self.app_data.clone(),
            user_id: self.user_id,
        }
    }
}
#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize)]
pub struct ActiveSpeaker {
    pub producer_id: ProducerId,
    pub user_id: usize,
    pub volume: i8,
}

impl Clone for ActiveSpeaker {
    fn clone(&self) -> ActiveSpeaker {
        Self {
            producer_id: self.producer_id,
            volume: self.volume,
            user_id: self.user_id,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TransportOptions {
    pub id: TransportId,
    pub dtls_parameters: DtlsParameters,
    pub ice_candidates: Vec<IceCandidate>,
    pub ice_parameters: IceParameters,
}
