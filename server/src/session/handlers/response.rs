use crate::session::socket_json_msg::ServerJsonMessage;
use crate::session::WsSession;
use actix::*;

impl Handler<ServerJsonMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ServerJsonMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap())
    }
}
