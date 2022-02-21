pub mod dto;
mod handlers;
pub mod message;
use actix::prelude::*;

use crate::chat::{BroadcastMessage, PrivateMessage};
use crate::media::app_data::media_service::MediaAppData;
use crate::media::app_data::transport::TransportAppData;
use crate::media::media_struct::ActiveSpeaker;
use crate::media::media_struct::DataProducer;
use crate::media::media_struct::TransportOptions;
use crate::room::dto::args::room_dto::RoomDto;
use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::room::dto::new_user_response::NewUserResponse;
use crate::room::dto::new_webrtc_transport_response::NewWebrtcTransportResponse;
use crate::server::Server;
use crate::session::room_actor_msg::DisconnectMessage;
use crate::session::WsSession;
use crate::user::User;
use async_rwlock::RwLock;
use mediasoup::audio_level_observer::{AudioLevelObserver, AudioLevelObserverVolume};
use mediasoup::data_structures::AppData;
use mediasoup::data_structures::TransportListenIp;
use mediasoup::producer::ProducerId;
use mediasoup::router::PipeToRouterOptions;
use mediasoup::router::{Router, RouterId};
use mediasoup::transport::Transport;
use mediasoup::webrtc_transport::{TransportListenIps, WebRtcTransportOptions};
use message::{CloseInactiveRoomMessage, RoomActiveSpeakerDetector};
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
///
/// Room struct maintains users , public/private messages, consumer/producer routers.
///
///
pub struct Room {
    id: String,
    // gateway room creator addr
    server_addr: Addr<Server>,
    // users
    users: HashMap<usize, Rc<RwLock<User>>>,
    // public chat messages
    messages: Vec<BroadcastMessage>,
    // private messages
    private_messages: Vec<PrivateMessage>,
    consumer_routers: HashMap<RouterId, Router>,
    producer_routers: HashMap<RouterId, Router>,
    router_producers: HashMap<RouterId, Vec<ProducerId>>,
    current_producer_id: RouterId,
    current_consumer_id: RouterId,
    active_speaker_elected: Option<ActiveSpeaker>,
    // user addresses, additional storage for audio_level
    user_addresses: HashMap<usize, Addr<WsSession>>,
    audio_level_observers: HashMap<RouterId, AudioLevelObserver>,
}

impl Room {
    pub fn new(param: RoomDto) -> Result<Room, std::io::ErrorKind> {
        let mut user_addresses = HashMap::new();
        user_addresses.insert(param.admin.id, param.admin.ws_actor_addr.clone());
        let mut users = HashMap::new();
        users.insert(param.admin.id, Rc::new(RwLock::new(param.admin)));
        let current_consumer_id = param.consumer_router.id();
        let current_producer_id = param.producer_router.id();
        let mut consumer_routers = HashMap::new();
        consumer_routers.insert(current_consumer_id, param.consumer_router);
        let mut producer_routers = HashMap::new();
        producer_routers.insert(current_producer_id, param.producer_router);
        let mut audio_level_observers = HashMap::new();
        audio_level_observers.insert(current_producer_id, param.audio_level_observer);
        Ok(Room {
            id: param.id,
            private_messages: vec![],
            users,
            messages: vec![],
            server_addr: param.server_addr,
            current_producer_id,
            current_consumer_id,
            producer_routers,
            router_producers: HashMap::new(),
            consumer_routers,
            user_addresses,
            audio_level_observers,
            active_speaker_elected: None,
        })
    }
    ///
    /// create webrtc transport using listen ip and announced ip specified in environment variable.
    ///
    pub async fn create_webrtc_transport(
        user: Rc<RwLock<User>>,
        router: Router,
        is_consumer: bool,
    ) {
        let listen_ip = env::var("LISTEN_IP").unwrap();
        let announced_ip: String = env::var("ANNOUNCED_IP").unwrap();
        let arr: Vec<&str> = announced_ip.split(".").collect();
        let mut transport_options =
            WebRtcTransportOptions::new(TransportListenIps::new(TransportListenIp {
                ip: listen_ip.parse().unwrap(),
                announced_ip: Some(IpAddr::V4(Ipv4Addr::new(
                    arr[0].parse().unwrap(),
                    arr[1].parse().unwrap(),
                    arr[2].parse().unwrap(),
                    arr[3].parse().unwrap(),
                ))),
            }));
        transport_options.enable_tcp = true;
        transport_options.enable_udp = true;
        transport_options.prefer_tcp = true;
        if is_consumer {
            transport_options.app_data = AppData::new(TransportAppData {
                router_id: router.id(),
                is_consumer: true,
            });
            let user_addr = { user.as_ref().read().await.ws_actor_addr.clone() };
            let id = { user.as_ref().read().await.id };
            let consumer_transport = match router.create_webrtc_transport(transport_options).await {
                Ok(transport) => transport,
                Err(error) => {
                    user_addr.do_send(DisconnectMessage {
                        id,
                        send_to_client: true,
                        send_to_room: true,
                        message: error.to_string(),
                    });
                    return;
                }
            };
            user_addr.do_send(NewWebrtcTransportResponse {
                transport_type: String::from("consumer"),
                webrtc_transport: TransportOptions {
                    id: consumer_transport.id(),
                    dtls_parameters: consumer_transport.dtls_parameters(),
                    ice_candidates: consumer_transport.ice_candidates().clone(),
                    ice_parameters: consumer_transport.ice_parameters().clone(),
                },
            });
            {
                user.as_ref()
                    .write()
                    .await
                    .insert_transport(consumer_transport, true);
            }
        } else {
            transport_options.app_data = AppData::new(TransportAppData {
                router_id: router.id(),
                is_consumer: false,
            });
            let user_addr = { user.as_ref().read().await.ws_actor_addr.clone() };
            let id = { user.as_ref().read().await.id };
            let producer_transport = match router
                .create_webrtc_transport(transport_options.clone())
                .await
            {
                Ok(transport) => transport,
                Err(error) => {
                    user_addr.do_send(DisconnectMessage {
                        id,
                        send_to_client: true,
                        send_to_room: true,
                        message: error.to_string(),
                    });
                    return;
                }
            };
            user_addr.do_send(NewWebrtcTransportResponse {
                transport_type: String::from("producer"),
                webrtc_transport: TransportOptions {
                    id: producer_transport.id(),
                    dtls_parameters: producer_transport.dtls_parameters(),
                    ice_candidates: producer_transport.ice_candidates().clone(),
                    ice_parameters: producer_transport.ice_parameters().clone(),
                },
            });
            {
                user.as_ref()
                    .write()
                    .await
                    .insert_transport(producer_transport, false);
            }
        }
    }
    pub async fn send_message(users: &HashMap<usize, Rc<RwLock<User>>>, message: BroadcastMessage) {
        for user_lock in users.values() {
            let user = user_lock.as_ref().read().await;
            if user.id != message.sender_id {
                user.ws_actor_addr.do_send(BroadcastMessageResponse {
                    id: message.id,
                    message: message.message.clone(),
                    user_name: message.user_name.clone(),
                    sender_id: message.sender_id,
                    timestamp: message.timestamp,
                });
            }
        }
    }
    ///
    /// selects active speaker each 2* AUDIO_INTERVAL_MSC miliseonds.
    ///
    fn heart_beat(&mut self, ctx: &mut Context<Self>) {
        let interval: u64 = env::var("AUDIO_INTERVAL_MSC").unwrap().parse().unwrap();
        ctx.run_interval(Duration::from_millis(interval * 2), |act, ctx| {
            if act.active_speaker_elected.is_some() {
                for addr in act.user_addresses.values() {
                    addr.do_send(act.active_speaker_elected.as_ref().unwrap().clone());
                }
                act.active_speaker_elected = None;
            }
            if act.users.len() == 0 {
                ctx.stop();
            }
        });
    }

    pub async fn notify_join_user(
        users: &HashMap<usize, Rc<RwLock<User>>>,
        msg: NewUserResponse,
        skip_id: usize,
    ) {
        for user_lock in users.values() {
            let user = user_lock.as_ref().read().await;
            if user.id != skip_id {
                user.ws_actor_addr.do_send(msg.clone());
            }
        }
    }
    ///
    /// This function creates missing webrtc transports when application scales up.
    ///
    pub async fn create_missing_transports(
        users: &Vec<Rc<RwLock<User>>>,
        router: Router,
        is_consumer: bool,
    ) {
        let router_id = router.id();
        for user_rw_lock in users {
            let mut transport_exist = false;
            {
                let user = user_rw_lock.as_ref().read().await;
                for transport in user.get_transports(true) {
                    let transport_router_id = transport
                        .app_data()
                        .deref()
                        .clone()
                        .downcast::<TransportAppData>()
                        .unwrap()
                        .as_ref()
                        .router_id;
                    if transport_router_id == router_id {
                        transport_exist = true
                    }
                }
            }
            if !transport_exist {
                println!("creating_webrtc_trasnrpot for missed people");
                Room::create_webrtc_transport(user_rw_lock.clone(), router.clone(), is_consumer)
                    .await
            }
        }
    }
    ///
    /// This function returns producers that each user has.
    ///
    pub async fn get_producer_users(users: &HashMap<usize, Rc<RwLock<User>>>) -> Vec<DataProducer> {
        let mut prod: Vec<DataProducer> = vec![];
        for user_lock in users.values() {
            let user = user_lock.as_ref().read().await;
            let user_id = user.id;
            for producer in user.get_producers() {
                let app_data = producer
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<MediaAppData>()
                    .unwrap()
                    .as_ref()
                    .clone();
                prod.push(DataProducer {
                    id: producer.id(),
                    app_data,
                    user_id,
                });
            }
        }
        return prod;
    }
    ///
    /// This function is used to pipe specified producer's stream from source router to destination router.
    ///
    async fn pipe_to_router(src_router: Router, dest_router: Router, producer_id: ProducerId) {
        let options = PipeToRouterOptions::new(dest_router.clone());
        match src_router
            .pipe_producer_to_router(producer_id, options)
            .await
        {
            Ok(_) => {
                println!(
                    "producer {:?} piped to router: {:?}",
                    producer_id,
                    dest_router.id()
                );
            }
            Err(error) => {
                println!("error while pipe_to_router {:?}", error);
            }
        }
    }
}
///
/// Room actor maintains state of a room.
///
impl Actor for Room {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heart_beat(ctx);
        let address = ctx.address().clone();
        let audio_level_observer = self
            .audio_level_observers
            .get(&self.current_producer_id)
            .unwrap();
        audio_level_observer
            .on_silence(|| {
                println!("It's silenceeeeee");
            })
            .detach();
        audio_level_observer
            .on_volumes(move |voloumes: &Vec<AudioLevelObserverVolume>| {
                if voloumes.len() > 0 {
                    // loudest producer_id
                    let producer_id = voloumes[0].producer.id();
                    let app_data = voloumes[0].producer.app_data().deref().clone();
                    if let Ok(app) = app_data.downcast::<MediaAppData>() {
                        let volume = voloumes[0].volume;
                        address.do_send(RoomActiveSpeakerDetector {
                            producer_id,
                            volume,
                            user_id: app.user_id,
                        });
                    } else {
                        println!("Error While downcasting data app");
                    }
                }
            })
            .detach();
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.consumer_routers.clear();
        self.producer_routers.clear();
        self.server_addr
            .do_send(CloseInactiveRoomMessage(self.id.clone()));
        Running::Stop
    }
}
