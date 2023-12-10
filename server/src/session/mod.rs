pub mod client_msg;
pub mod dto;
mod internal_msg;
pub mod room_actor_msg;
pub mod socket_json_msg;

use crate::room::dto::rtp_capability_settings_request::RtpCapabilitiesSettings;
use crate::room::Room;
use crate::server::Server;
use actix::*;
use actix_web_actors::ws;
use client_msg::JoinRoomRequest;
use internal_msg::InternalErrorMessage;
use rand::{self, rngs::ThreadRng, Rng};
use room_actor_msg::{DisconnectMessage, JoinRoomMessage};
use socket_json_msg::ClientJsonMessage;
use std::time::{Duration, Instant};
use tracing::info;

mod handlers;
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
pub struct WsSession {
    /// unique session id
    pub session_id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,
    /// joined room
    pub room: Option<String>,
    pub rng: ThreadRng,
    /// peer name
    pub id: usize,
    /// Chat server
    pub room_addr: Option<Addr<Room>>,

    pub server_addr: Addr<Server>,
    // if session is authenticated
    pub is_login: bool,
}
///
/// This actor is used to send user's messages retrieved from websocket to room's actor.
///
impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with Server
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.session_id = self.rng.gen::<usize>();
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        if self.room_addr.is_some() {
            self.room_addr.as_ref().unwrap().do_send(DisconnectMessage {
                id: self.id,
                message: String::from("User Actor Stopped"),
                send_to_client: false,
                send_to_room: true,
            });
        }
        Running::Stop
    }
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                info!("Websocket Client heartbeat failed, disconnecting!");
                ctx.notify(DisconnectMessage {
                    id: act.id,
                    message: String::from("session Timeout"),
                    send_to_client: true,
                    send_to_room: true,
                });
                return;
            }

            ctx.ping(b"");
        });
    }

    fn has_access(&self) -> bool {
        (self.is_login) && self.room_addr.is_some()
    }
    fn handle_message(&mut self, message: ClientJsonMessage, ctx: &mut ws::WebsocketContext<Self>) {
        match message {
            ClientJsonMessage::JoinRoom(JoinRoomRequest {
                room_id,
                user_name,
                user_id,
            }) => {
                if !room_id.is_empty() {
                    let addr = ctx.address();
                    self.id = user_id;
                    self.server_addr.do_send(JoinRoomMessage {
                        addr: addr.clone(),
                        user_id,
                        user_name,
                        room_id: room_id.clone(),
                    });
                } else {
                    ctx.notify(InternalErrorMessage::ServerFault(
                        "Room name should be more than 1 character".to_string(),
                    ));
                }
            }
            ClientJsonMessage::BroadcastMessage(broadcast_message_req) => {
                if self.has_access() && !broadcast_message_req.message.is_empty() {
                    self.room_addr
                        .as_ref()
                        .unwrap()
                        .do_send(broadcast_message_req);
                } else {
                    ctx.notify(InternalErrorMessage::UnAuthorized(
                        "due to not authorized or no room selected.".to_string(),
                    ));
                }
            }
            ClientJsonMessage::PrivateChat(private_message_req) => {
                if self.has_access() {
                    self.room_addr
                        .as_ref()
                        .unwrap()
                        .do_send(private_message_req);
                } else {
                    ctx.notify(InternalErrorMessage::UnAuthorized(
                        "due to not authorized or no room selected.".to_string(),
                    ));
                }
            }
            ClientJsonMessage::SetRtpCapability(RtpCapabilitiesSettings {
                user_id,
                rtp_capabilities,
            }) => {
                if self.has_access() {
                    self.room_addr
                        .as_ref()
                        .unwrap()
                        .do_send(RtpCapabilitiesSettings {
                            user_id,
                            rtp_capabilities,
                        })
                } else {
                    ctx.notify(InternalErrorMessage::UnAuthorized(
                        "due to not authorized or no room selected.".to_string(),
                    ));
                }
            }
            ClientJsonMessage::ConnectConsumerTransport(cc_transport_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(cc_transport_req);
                } else {
                    ctx.notify(InternalErrorMessage::UnAuthorized(
                        "due to not authorized or no room selected.".to_string(),
                    ));
                }
            }
            ClientJsonMessage::ConnectProducerTransport(cp_transport_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(cp_transport_req);
                } else {
                    ctx.notify(InternalErrorMessage::UnAuthorized(
                        "due to not authorized or no room selected.".to_string(),
                    ));
                }
            }
            ClientJsonMessage::Produce(produce_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(produce_req);
                }
            }
            ClientJsonMessage::ProducerResume(produce_resume_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(produce_resume_req);
                }
            }
            ClientJsonMessage::ProducerPause(pause_producer_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(pause_producer_req);
                }
            }
            ClientJsonMessage::ProducerClose(close_producer_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(close_producer_req);
                }
            }
            ClientJsonMessage::ConsumerResume(consumer_resume_req) => {
                if self.has_access() {
                    self.room_addr
                        .as_ref()
                        .unwrap()
                        .do_send(consumer_resume_req);
                }
            }
            ClientJsonMessage::ConsumerPause(consumer_pause_req) => {
                if self.has_access() {
                    self.room_addr.as_ref().unwrap().do_send(consumer_pause_req);
                }
            }

            _ => {
                ctx.notify(InternalErrorMessage::NotFound(
                    "Method not found.".to_string(),
                ));
            }
        }
    }
}
