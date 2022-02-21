use crate::media::app_data::router::{ConsumerRouterAppData, ProducerRouterAppData};
use crate::media::app_data::worker::{ConsumerWorkerAppData, ProducerWorkerAppData};
use crate::room::dto::join_room_response::JoinRoomActorResponse;
use crate::room::Room;
use crate::session::WsSession;
use crate::user::UserDto;
use actix::prelude::*;
use async_rwlock::RwLock;
use mediasoup::router::Router;
use mediasoup::worker::{Worker, WorkerId};
use std::sync::Arc;

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemovedConsumer {
    pub consumer_router_app_data: Arc<RwLock<ConsumerRouterAppData>>,
    pub consumer_worker_app_data: Arc<RwLock<ConsumerWorkerAppData>>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddConsumer {
    pub consumer_router_app_data: Arc<RwLock<ConsumerRouterAppData>>,
    pub consumer_worker_app_data: Arc<RwLock<ConsumerWorkerAppData>>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct RemovedProducer {
    pub producer_router_app_data: Arc<RwLock<ProducerRouterAppData>>,
    pub producer_worker_app_data: Arc<RwLock<ProducerWorkerAppData>>,
}
#[derive(Message)]
#[rtype(result = "()")]
pub struct AddProducer {
    pub producer_router_app_data: Arc<RwLock<ProducerRouterAppData>>,
    pub producer_worker_app_data: Arc<RwLock<ProducerWorkerAppData>>,
    pub threashold: usize,
}

#[derive(Message)]
#[rtype(result = "JoinRoomActorResponse")]
pub struct RoomUserEntranceMessage {
    pub user: UserDto,
    pub addr: Addr<WsSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AsyncRoomCreation {
    pub arb: Arbiter,
    pub room_id: String,
    pub room_addr: Addr<Room>,
}

#[derive(Message)]
#[rtype(result = "Option<Worker>")]
pub struct FindLeastWorkerUsage {
    pub is_consumer: bool,
    pub min_consumer_numbers: usize,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct CanHandleConsumer {
    pub number_consumers: usize,
    pub is_consumer: bool,
    pub worker_pid: WorkerId,
}

#[derive(Message, Clone, Copy)]
#[rtype(result = "Worker")]
pub struct CreateWorker {
    pub is_consumer: bool,
}

#[derive(Message)]
#[rtype(result = "Router")]
pub struct CreateRouter {
    pub is_consumer: bool,
    pub worker: Option<Worker>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveWorker {
    pub is_consumer: bool,
    pub worker_id: WorkerId,
}
