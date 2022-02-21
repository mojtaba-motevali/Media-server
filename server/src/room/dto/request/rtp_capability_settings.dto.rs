use actix::*;
use mediasoup::rtp_parameters::RtpCapabilities;
use serde::Deserialize;

#[derive(Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct RtpCapabilitiesSettings {
    pub rtp_capabilities: RtpCapabilities,
    pub user_id: usize,
}
