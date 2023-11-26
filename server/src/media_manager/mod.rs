use crate::media::app_data::app_traits::{AppDataOperation, WorkerAppDataOperation};
use crate::media::app_data::router::{ConsumerRouterAppData, ProducerRouterAppData};
use crate::media::app_data::worker::{ConsumerWorkerAppData, ProducerWorkerAppData};
use crate::media::codecs::media_codecs;
use crate::server::message::{AddConsumer, AddProducer, RemovedConsumer, RemovedProducer};
use crate::server::Server;
use actix::Addr;
use async_rwlock::RwLock;
use mediasoup::consumer::Consumer;
use mediasoup::data_structures::AppData;
use mediasoup::producer::Producer;
use mediasoup::router::{NewTransport, Router, RouterOptions};
use mediasoup::transport::Transport;
use mediasoup::webrtc_transport::WebRtcTransport;
use mediasoup::worker::{Worker, WorkerId, WorkerLogLevel, WorkerSettings};
use mediasoup::worker_manager::WorkerManager;
use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::sync::Arc;
type ArcMutexHashMap = HashMap<WorkerId, Worker>;
///
/// Worker manager struct is used to manage mediasoup rotuers/workers.
/// also maintains following state in each worker and router.
/// see ConsumerWorkerAppData,ProducerWorkerAppData,ConsumerRouterAppData,ProducerRouterAppData
///
#[derive(Clone)]
pub struct MediaWorkerManager {
    server_addr: Option<Addr<Server>>,
    manager: WorkerManager,
    max_limit_producers: usize,
    max_limit_consumers: usize,
    current_producer_worker_id: Option<WorkerId>,
    current_consumer_worker_id: Option<WorkerId>,
    producer_workers: ArcMutexHashMap,
    consumer_workers: ArcMutexHashMap,
}

impl MediaWorkerManager {
    pub fn new(manager: WorkerManager) -> Self {
        let producer_workers = HashMap::new();
        let consumer_workers = HashMap::new();
        let current_consumer_worker_id = None;
        let current_producer_worker_id = None;
        let max_limit_consumers: usize = env::var("WORKER_CONSUMER_MAX")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let max_limit_producers: usize = env::var("WORKER_PRODUCER_MAX")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        Self {
            server_addr: None,
            max_limit_producers,
            max_limit_consumers,
            manager,
            producer_workers,
            consumer_workers,
            current_consumer_worker_id,
            current_producer_worker_id,
        }
    }

    pub async fn clean_workers<T: Send + Sync + 'static + Copy + WorkerAppDataOperation>(
        &mut self,
        is_consumer: bool,
    ) {
        let mut keys_to_remove = vec![];
        let workers = self.get_workers(is_consumer);
        let worker_length = workers.len();
        for worker in workers {
            let worker_app_data = worker
                .app_data()
                .deref()
                .clone()
                .downcast::<RwLock<T>>()
                .unwrap();
            let app_data = worker_app_data.read().await;
            if app_data.get_number_of_routers() < 1 {
                keys_to_remove.push(worker.id());
            }
        }
        let mut number_to_remove = keys_to_remove.len();
        if number_to_remove == worker_length {
            number_to_remove -= 1;
        }
        for i in 0..number_to_remove {
            self.remove_worker(keys_to_remove[i], is_consumer);
        }
    }

    pub async fn run(&mut self, address: Addr<Server>) {
        self.server_addr.replace(address);
        let worker: Worker = self.create_worker(true).await;
        let _ = self.insert_to_consumers(worker);
        let worker: Worker = self.create_worker(false).await;
        let _ = self.insert_to_producers(worker);
    }
    async fn find_least<T: Send + Sync + 'static + AppDataOperation + Copy>(
        &self,
        map: ArcMutexHashMap,
        threashold: usize,
    ) -> Option<Worker> {
        let mut max = 500;
        let mut max_index: Option<WorkerId> = None;
        for (key, worker) in map.iter() {
            let lock = worker
                .app_data()
                .deref()
                .clone()
                .downcast::<RwLock<T>>()
                .unwrap();
            let data = lock.read().await;
            if data.get() < max {
                max = data.get();
                max_index.replace(key.clone());
            }
        }
        if max < threashold {
            Some(map.get(&max_index.unwrap()).unwrap().clone())
        } else {
            None
        }
    }

    pub fn get_workers(&self, is_consumer: bool) -> Vec<Worker> {
        let mut vector = vec![];
        if is_consumer {
            for worker in self.consumer_workers.iter() {
                vector.push(worker.1.clone());
            }
        } else {
            for worker in self.producer_workers.iter() {
                vector.push(worker.1.clone());
            }
        }
        vector
    }

    pub async fn find_least_pworker(&self, threashold: usize) -> Option<Worker> {
        self.find_least::<ProducerWorkerAppData>(self.producer_workers.clone(), threashold)
            .await
    }
    pub async fn find_least_cworker(&self, threashold: usize) -> Option<Worker> {
        self.find_least::<ConsumerWorkerAppData>(self.consumer_workers.clone(), threashold)
            .await
    }
    pub fn get_consumer_worker(&self) -> Worker {
        let worker = self
            .consumer_workers
            .get(&self.current_consumer_worker_id.unwrap());
        return worker.unwrap().clone();
    }
    pub fn get_producer_worker(&self) -> Worker {
        let worker = self
            .producer_workers
            .get(&self.current_producer_worker_id.unwrap());
        return worker.unwrap().clone();
    }
    pub fn remove_worker(&mut self, worker_id: WorkerId, is_consumer: bool) -> Option<Worker> {
        if is_consumer {
            self.consumer_workers.remove(&worker_id)
        } else {
            self.producer_workers.remove(&worker_id)
        }
    }
    pub fn insert_to_producers(&mut self, worker: Worker) -> Worker {
        self.current_producer_worker_id.replace(worker.id());
        self.producer_workers.insert(worker.id(), worker.clone());
        worker
    }
    pub fn insert_to_consumers(&mut self, worker: Worker) -> Worker {
        self.current_consumer_worker_id.replace(worker.id());
        self.consumer_workers.insert(worker.id(), worker.clone());
        worker
    }
    pub async fn create_consumer_router(&self, worker: Worker) -> Router {
        let mut settings: RouterOptions = RouterOptions::new(media_codecs());
        settings.app_data = AppData::new(RwLock::new(ConsumerRouterAppData::new(worker.id())));
        let router: Router = worker.create_router(settings).await.unwrap();
        let router_clone: Router = router.clone();
        let address = self.server_addr.clone().unwrap();
        router
            .on_new_transport(move |transport: NewTransport| match transport {
                NewTransport::WebRtc(transport) => {
                    MediaWorkerManager::register_transport_consumer(
                        transport,
                        &router_clone,
                        &worker,
                        address.clone(),
                    );
                }
                _ => {}
            })
            .detach();
        return router;
    }

    fn set_router_events<T: Send + Sync + 'static + Copy + WorkerAppDataOperation>(
        worker: &Worker,
    ) {
        let worker_app_data = worker
            .app_data()
            .deref()
            .clone()
            .downcast::<RwLock<T>>()
            .unwrap()
            .clone();
        let app_data_outer = worker_app_data.clone();
        worker
            .on_new_router(move |router: &Router| {
                let app_data = app_data_outer.clone();
                if actix::Arbiter::is_running() {
                    actix::spawn(async move {
                        let mut app = app_data.write().await;
                        app.add_router();
                    });
                }
                let app_data = app_data_outer.clone();
                router
                    .on_close(move || {
                        if actix::Arbiter::is_running() {
                            actix::spawn(async move {
                                let mut app = app_data.write().await;
                                app.subtract_router();
                            });
                        }
                    })
                    .detach();
            })
            .detach();
    }

    pub async fn create_producer_router(&self, worker: Worker) -> Router {
        let mut settings: RouterOptions = RouterOptions::new(media_codecs());
        settings.app_data = AppData::new(RwLock::new(ProducerRouterAppData::new(worker.id())));
        let router: Router = worker.create_router(settings).await.unwrap();
        let router_clone: Router = router.clone();
        let address = self.server_addr.clone().unwrap();
        let threashold = self.max_limit_producers;
        router
            .on_new_transport(move |transport: NewTransport| {
                let _addr = address.clone();
                match transport {
                    NewTransport::WebRtc(transport) => {
                        MediaWorkerManager::register_transport_producer(
                            transport,
                            &router_clone,
                            &worker,
                            _addr,
                            threashold,
                        );
                    }
                    _ => {}
                }
            })
            .detach();
        return router;
    }

    pub async fn create_worker(&self, is_consumer: bool) -> Worker {
        let mut setting: WorkerSettings = WorkerSettings::default();
        setting.log_level = WorkerLogLevel::Debug;
        if is_consumer {
            setting.app_data = AppData::new(RwLock::new(ConsumerWorkerAppData::new()));
            let worker: Worker = self.manager.create_worker(setting).await.unwrap();
            MediaWorkerManager::set_router_events::<ConsumerWorkerAppData>(&worker);
            worker
        } else {
            setting.app_data = AppData::new(RwLock::new(ProducerWorkerAppData::new()));
            let worker: Worker = self.manager.create_worker(setting).await.unwrap();
            MediaWorkerManager::set_router_events::<ProducerWorkerAppData>(&worker);
            worker
        }
    }
    pub async fn can_handle_consumers(
        &self,
        pid: WorkerId,
        total_consumers: usize,
        is_consumer: bool,
    ) -> bool {
        if is_consumer {
            if let Some(wk_consumer) = self.consumer_workers.get(&pid).as_ref() {
                let temp = wk_consumer
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<RwLock<ConsumerWorkerAppData>>()
                    .unwrap();
                let app_data = temp.read().await;
                return self.max_limit_consumers > app_data.deref().get() + total_consumers;
            }
            return false;
        } else {
            if let Some(wk_producer) = self.producer_workers.get(&pid).as_ref() {
                let temp = wk_producer
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<RwLock<ProducerWorkerAppData>>()
                    .unwrap();
                let app_data = temp.read().await;
                return self.max_limit_producers > app_data.deref().get() + total_consumers;
            }
            return false;
        }
    }
    pub async fn create_router(&self, is_consumer: bool) -> Router {
        if is_consumer {
            let worker = self.get_consumer_worker();
            return self.create_consumer_router(worker).await;
        } else {
            let worker = self.get_producer_worker();
            return self.create_producer_router(worker).await;
        }
    }
    fn register_transport_consumer(
        transport: &WebRtcTransport,
        router: &Router,
        worker: &Worker,
        address: Addr<Server>,
    ) {
        let arc_mutex_cw = worker
            .app_data()
            .deref()
            .clone()
            .downcast::<RwLock<ConsumerWorkerAppData>>()
            .unwrap();
        let arc_mutex_cr = router
            .app_data()
            .deref()
            .clone()
            .downcast::<RwLock<ConsumerRouterAppData>>()
            .unwrap();
        let router_app_data = arc_mutex_cr.clone();
        let worker_app_data = arc_mutex_cw.clone();

        transport
            .on_new_consumer(Arc::new(move |consumer: &Consumer| {
                let rapp_data = router_app_data.clone();
                let wapp_data = worker_app_data.clone();
                address.do_send(AddConsumer {
                    consumer_worker_app_data: wapp_data.clone(),
                    consumer_router_app_data: rapp_data.clone(),
                });
                let _address = address.clone();
                consumer
                    .on_close(Box::new(move || {
                        _address.do_send(RemovedConsumer {
                            consumer_worker_app_data: wapp_data,
                            consumer_router_app_data: rapp_data,
                        });
                    }))
                    .detach();
            }))
            .detach();
    }

    fn register_transport_producer(
        transport: &WebRtcTransport,
        router: &Router,
        worker: &Worker,
        address: Addr<Server>,
        threashold: usize,
    ) {
        let worker_app_data: Arc<RwLock<ProducerWorkerAppData>> = worker
            .app_data()
            .deref()
            .clone()
            .downcast::<RwLock<ProducerWorkerAppData>>()
            .unwrap();
        let router_app_data: Arc<RwLock<ProducerRouterAppData>> = router
            .app_data()
            .deref()
            .clone()
            .downcast::<RwLock<ProducerRouterAppData>>()
            .unwrap();
        transport
            .on_new_producer(Arc::new(move |producer: &Producer| {
                let rapp_data: Arc<RwLock<ProducerRouterAppData>> = router_app_data.clone();
                let wapp_data = worker_app_data.clone();
                address.do_send(AddProducer {
                    producer_worker_app_data: wapp_data.clone(),
                    producer_router_app_data: rapp_data.clone(),
                    threashold: threashold,
                });
                let _address = address.clone();
                producer
                    .on_close(move || {
                        _address.do_send(RemovedProducer {
                            producer_worker_app_data: wapp_data,
                            producer_router_app_data: rapp_data,
                        });
                    })
                    .detach();
            }))
            .detach();
    }
}
