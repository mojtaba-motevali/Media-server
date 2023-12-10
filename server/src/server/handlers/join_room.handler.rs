use crate::room::dto::args::room_dto::RoomDto;
use crate::room::dto::join_room_response::JoinRoomActorResponse;
use crate::room::Room;
use crate::server::message::{
    AsyncRoomCreation, CreateRouter, CreateWorker, FindLeastWorkerUsage, RoomUserEntranceMessage,
};
use crate::server::Server;
use crate::session::room_actor_msg::DisconnectMessage;
use crate::session::room_actor_msg::JoinRoomMessage;
use crate::user::{User, UserDto};
use actix::prelude::*;
use mediasoup::audio_level_observer::{AudioLevelObserver, AudioLevelObserverOptions};
use mediasoup::router::Router;
use std::env;
use tracing::info;

///
/// This handler is main point where gets redirected when joins. depending on user's requested room,
/// this handler either creates a room and joins user to it or joins users to requested room if it exists.
/// This handler also checks the limiation of the current worker thread and creates additional thread if needed.
///
impl Handler<JoinRoomMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: JoinRoomMessage, ctx: &mut Context<Self>) -> Self::Result {
        let JoinRoomMessage {
            addr,
            user_id,
            user_name,
            room_id,
        } = msg;
        let room_id_2 = room_id.clone();
        let server_addr = ctx.address().clone();
        for room in self.rooms.iter() {
            if *(room.0) != room_id {
                room.1.do_send(DisconnectMessage {
                    id: user_id,
                    send_to_client: true,
                    send_to_room: true,
                    message: String::from("You've been disconneed by another session."),
                });
            }
        }
        if self.rooms.get(&room_id).is_none() {
            let ctx_addr = ctx.address().clone();
            actix::spawn(async move {
                let consumer_router: Router;
                let least_worker = ctx_addr
                    .send(FindLeastWorkerUsage {
                        is_consumer: true,
                        min_consumer_numbers: 1,
                    })
                    .await
                    .unwrap();
                if least_worker.is_some() {
                    consumer_router = ctx_addr
                        .send(CreateRouter {
                            is_consumer: true,
                            worker: least_worker,
                        })
                        .await
                        .unwrap();
                } else {
                    let worker = ctx_addr
                        .send(CreateWorker { is_consumer: true })
                        .await
                        .unwrap();
                    consumer_router = ctx_addr
                        .send(CreateRouter {
                            worker: Some(worker),
                            is_consumer: true,
                        })
                        .await
                        .unwrap();
                }
                let producer_router: Router = ctx_addr
                    .send(CreateRouter {
                        is_consumer: false,
                        worker: None,
                    })
                    .await
                    .unwrap();
                let mut audio_observ_options = AudioLevelObserverOptions::default();
                audio_observ_options.interval =
                    env::var("AUDIO_INTERVAL_MSC").unwrap().parse().unwrap();
                let audio_level_observer: AudioLevelObserver = match producer_router
                    .create_audio_level_observer(audio_observ_options)
                    .await
                {
                    Ok(obser) => obser,
                    Err(err) => {
                        let message = "error while creating audioLevelObserver ".to_string()
                            + &err.to_string();
                        info!("{}", message);
                        addr.do_send(DisconnectMessage {
                            message,
                            send_to_client: true,
                            send_to_room: false,
                            id: user_id,
                        });
                        return;
                    }
                };
                let _room_id = room_id_2.clone();
                let router_rtp_capabilities = producer_router.rtp_capabilities().clone();
                let _name = user_name.clone();
                let _addr = addr.clone();
                let room_addr = {
                    let admin = User::new(user_id, _name, addr);

                    let async_room = Room::new(RoomDto {
                        audio_level_observer,
                        id: _room_id,
                        admin,
                        server_addr,
                        consumer_router: consumer_router.clone(),
                        producer_router: producer_router.clone(),
                    })
                    .unwrap();
                    async_room.start()
                };
                ctx_addr.do_send(AsyncRoomCreation {
                    room_id: room_id_2,
                    arb: Arbiter::new(),
                    room_addr: room_addr.clone(),
                });
                _addr.do_send(JoinRoomActorResponse {
                    room_addr,
                    messages: vec![],
                    users: vec![UserDto {
                        id: Some(user_id),
                        name: Some(user_name),
                    }],
                    router_rtp_capabilities,
                });
            });
        } else {
            let room = self.rooms.get(&room_id).unwrap().clone();
            let _addr = addr.clone();
            actix::spawn(async move {
                let user_dto = UserDto {
                    id: Some(user_id),
                    name: Some(user_name),
                };
                let object = match room
                    .send(RoomUserEntranceMessage {
                        user: user_dto,
                        addr,
                    })
                    .await
                {
                    Ok(res) => res,
                    Err(err) => {
                        let message = "error while joining user ".to_string() + &err.to_string();
                        info!("{}", message);
                        _addr.do_send(DisconnectMessage {
                            message,
                            send_to_client: true,
                            send_to_room: false,
                            id: user_id,
                        });
                        // addr.do_send()
                        return;
                    }
                };
                _addr.do_send(object);
            });
        }
    }
}
