use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::user::UserDto;
use mediasoup::rtp_parameters::RtpCapabilitiesFinalized;
use serde::Serialize;

#[derive(Serialize)]
pub struct JoinRoomResponse {
    pub users: Vec<UserDto>,
    pub messages: Vec<BroadcastMessageResponse>,
    pub router_rtp_capabilities: RtpCapabilitiesFinalized,
}
