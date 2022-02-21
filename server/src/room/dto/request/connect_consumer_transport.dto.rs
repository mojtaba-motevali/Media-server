use actix::*;
use mediasoup::data_structures::DtlsParameters;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ConnectConsumerTransportRequest {
    pub user_id: usize,
    pub transport_id: String,
    pub dtls_parameters: DtlsParameters,
}
