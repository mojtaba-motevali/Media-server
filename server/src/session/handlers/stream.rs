use crate::session::internal_msg::InternalErrorMessage;
use crate::session::room_actor_msg::DisconnectMessage;
use crate::session::socket_json_msg::ClientJsonMessage;
use actix::*;
use actix_web_actors::ws;
use std::time::Instant;
use tracing::info;

use crate::session::WsSession;

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let message: ClientJsonMessage = match serde_json::from_slice(text.as_bytes()) {
                    Ok(struc) => struc,
                    Err(_err) => {
                        info!("ws:text {:?}", _err);
                        ctx.notify(InternalErrorMessage::InvalidJson(
                            "Json structure didn't match\"type\" or was not json at all."
                                .to_string(),
                        ));
                        return;
                    }
                };
                self.handle_message(message, ctx);
            }
            ws::Message::Binary(_) => info!("Unexpected binary"),
            ws::Message::Close(reason) => {
                info!("{:?}", reason);
                ctx.address().do_send(DisconnectMessage {
                    id: self.session_id,
                    send_to_client: false,
                    send_to_room: true,
                    message: String::from("Connection closed by User."),
                });
            }
            ws::Message::Continuation(_) => {
                ctx.notify(DisconnectMessage {
                    id: self.session_id,
                    send_to_client: true,
                    send_to_room: true,
                    message: String::from("Continuation reasons"),
                });
            }
            ws::Message::Nop => (),
        }
    }
}
