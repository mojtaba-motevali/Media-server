use actix::prelude::*;

use crate::media::media_struct::DataProducer;
use crate::session::WsSession;
use crate::user::UserDto;
use mediasoup::producer::ProducerId;
// new session
#[derive(Message)]
#[rtype(usize)]
pub struct ConnectMessage {
    pub addr: Addr<WsSession>,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct DisconnectMessage {
    pub id: usize,
    pub send_to_client: bool,
    pub send_to_room: bool,
    pub message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoomMessage {
    pub addr: Addr<WsSession>,
    pub user_name: String,
    pub user_id: usize,
    pub room_id: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewUser {
    pub user: UserDto,
}
impl Clone for NewUser {
    fn clone(&self) -> NewUser {
        Self {
            user: self.user.clone(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewProducer {
    pub data_producer: DataProducer,
    pub session_id: usize,
}
impl Clone for NewProducer {
    fn clone(&self) -> Self {
        Self {
            session_id: self.session_id,
            data_producer: self.data_producer.clone(),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseProducer {
    pub id: ProducerId,
    pub session_id: usize,
}
impl Clone for CloseProducer {
    fn clone(&self) -> CloseProducer {
        Self {
            id: self.id,
            session_id: self.session_id,
        }
    }
}
#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct PeerDisconnectMessage {
    pub user_id: usize,
}
