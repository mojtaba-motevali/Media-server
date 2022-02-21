use crate::server::Server;
use crate::user::User;
use actix::Addr;
use mediasoup::audio_level_observer::AudioLevelObserver;
use mediasoup::router::Router;

pub struct RoomDto {
    pub id: String,
    pub server_addr: Addr<Server>,
    pub admin: User,
    pub consumer_router: Router,
    pub producer_router: Router,
    pub audio_level_observer: AudioLevelObserver,
}
