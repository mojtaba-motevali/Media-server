use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::room::dto::private_message_response::PrivateMessageResponse;
use crate::session::socket_json_msg::ServerJsonMessage;
use crate::session::WsSession;
use actix::*;

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<BroadcastMessageResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessageResponse, ctx: &mut Self::Context) {
        ctx.notify(ServerJsonMessage::BroadcastMessage(msg));
    }
}
impl Handler<PrivateMessageResponse> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: PrivateMessageResponse, ctx: &mut Self::Context) {
        ctx.notify(ServerJsonMessage::PrivateChat(msg));
    }
}
