use super::Room;
use crate::chat::BroadcastMessage;
use crate::user::UserDto;
use actix::prelude::*;
use mediasoup::producer::ProducerId;
use mediasoup::rtp_parameters::RtpCapabilitiesFinalized;

#[derive(Message)]
#[rtype(result = "UserDto")]
#[derive(Debug)]
pub struct RoomActorUPRequestMessage();

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
pub struct CloseInactiveRoomMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoomActorResponse {
    pub users: Vec<UserDto>,
    pub room_addr: Addr<Room>,
    pub messages: Vec<BroadcastMessage>,
    pub router_rtp_capabilities: RtpCapabilitiesFinalized,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug, Copy)]
pub struct RoomActiveSpeakerDetector {
    pub producer_id: ProducerId,
    pub user_id: usize,
    pub volume: i8,
}

impl Clone for RoomActiveSpeakerDetector {
    fn clone(&self) -> Self {
        Self {
            producer_id: self.producer_id.clone(),
            user_id: self.user_id,
            volume: self.volume,
        }
    }
}
