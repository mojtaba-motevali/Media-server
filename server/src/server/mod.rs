pub mod handlers;
pub mod message;

use crate::media_manager::MediaWorkerManager;
use crate::room::Room;
use actix::prelude::*;
use async_rwlock::RwLock;
use mediasoup::worker_manager::WorkerManager;
use std::collections::HashMap;
use std::rc::Rc;
use tracing::info;
///
/// Server Actor is used to maintain internal mediasoup state and rooms addresses.
/// This actor can be used to manage rooms or achieve usage statistics of mediasoup state through
/// API used in Http server of actix.
///
pub struct Server {
    pub rooms: HashMap<String, Addr<Room>>,
    worker_manager: Rc<RwLock<MediaWorkerManager>>,
    arbiters: HashMap<String, Arbiter>,
}

impl Server {
    pub async fn new() -> Server {
        let manager = WorkerManager::new();
        let worker_manager = Rc::new(RwLock::new(MediaWorkerManager::new(manager)));
        Server {
            worker_manager,
            rooms: HashMap::new(),
            arbiters: HashMap::new(),
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Server Actor started");
        let worker_manager = self.worker_manager.clone();
        let address = _ctx.address().clone();
        actix::spawn(async move {
            worker_manager.as_ref().write().await.run(address).await;
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        info!("Server Actor Stopped");
        Running::Stop
    }
}
