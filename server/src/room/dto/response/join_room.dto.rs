use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::room::Room;
use crate::user::UserDto;
use actix::prelude::{Addr, Message};
use mediasoup::rtp_parameters::RtpCapabilitiesFinalized;

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoomActorResponse {
    pub users: Vec<UserDto>,
    pub room_addr: Addr<Room>,
    pub messages: Vec<BroadcastMessageResponse>,
    pub router_rtp_capabilities: RtpCapabilitiesFinalized,
}
