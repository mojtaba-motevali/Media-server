use crate::room::dto::join_room_response::JoinRoomActorResponse;
use crate::room::dto::new_user_response::NewUserResponse;
use crate::session::client_msg::UserDisconnect;
use crate::session::dto::join_room_response::JoinRoomResponse;
use crate::session::internal_msg::InternalErrorMessage;
use crate::session::room_actor_msg::{DisconnectMessage, PeerDisconnectMessage};
use crate::session::socket_json_msg::ServerJsonMessage;
use crate::session::WsSession;
use actix::*;

impl Handler<JoinRoomActorResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: JoinRoomActorResponse, ctx: &mut Self::Context) -> Self::Result {
        println!("user id : {:?} joined", self.id);
        self.is_login = true;
        self.room_addr.replace(msg.room_addr);
        ctx.notify(ServerJsonMessage::JoinRoom(JoinRoomResponse {
            messages: msg.messages,
            users: msg.users,
            router_rtp_capabilities: msg.router_rtp_capabilities,
        }));
    }
}

impl Handler<PeerDisconnectMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: PeerDisconnectMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::UserDisconnect(UserDisconnect {
            user_id: msg.user_id,
        }))
    }
}

impl Handler<NewUserResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: NewUserResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::NewUser(NewUserResponse {
            user: msg.user,
        }));
    }
}

impl Handler<DisconnectMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: DisconnectMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("user id {:?} disconnected.", msg.id);

        if self.room_addr.is_some() && msg.send_to_room {
            println!("informing room actor....");
            self.room_addr.as_ref().unwrap().do_send(DisconnectMessage {
                id: self.id,
                send_to_client: false,
                send_to_room: true,
                message: msg.message.clone(),
            })
        }
        if msg.send_to_client {
            ctx.notify(InternalErrorMessage::ServerFault(msg.message));
        }
        ctx.stop();
    }
}
