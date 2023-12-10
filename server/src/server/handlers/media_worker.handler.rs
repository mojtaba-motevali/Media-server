use crate::room::dto::internal::{InternalMessage, SaveRouter};
use crate::server::message::{
    CanHandleConsumer, CreateRouter, CreateWorker, FindLeastWorkerUsage, RemoveWorker,
};
use crate::server::Server;
use actix::prelude::*;
use mediasoup::router::Router;
use mediasoup::worker::Worker;
use tracing::error;
///
/// This handler is used to dump mediasoup worker and remove it from map.
///
impl Handler<RemoveWorker> for Server {
    type Result = ();
    fn handle(&mut self, msg: RemoveWorker, _ctx: &mut Context<Self>) -> Self::Result {
        let worker_manager = self.worker_manager.clone();
        let _ = Box::pin(
            async move {
                let mut wk = worker_manager.as_ref().write().await;
                if let Some(worker) = wk.remove_worker(msg.worker_id, msg.is_consumer) {
                    match worker.dump().await {
                        Ok(_worker_dump) => {}
                        Err(error) => {
                            error!("Worker Error on dumping {:?}", error.to_string());
                        }
                    }
                }
            }
            .into_actor(self),
        );
    }
}
///
/// This handler is used to determine whether mediasoup worker is capable of having provided
/// number of consumers in it's thread or not.
///
impl Handler<CanHandleConsumer> for Server {
    type Result = ResponseActFuture<Self, bool>;
    fn handle(&mut self, msg: CanHandleConsumer, _ctx: &mut Context<Self>) -> Self::Result {
        let worker_manager = self.worker_manager.clone();
        Box::pin(
            async move {
                worker_manager
                    .as_ref()
                    .read()
                    .await
                    .can_handle_consumers(msg.worker_pid, msg.number_consumers, msg.is_consumer)
                    .await
            }
            .into_actor(self)
            .map(|can_handle, _act, _ctx| can_handle),
        )
    }
}

///
/// This handler is used to find the least usage worker and returns it via pining it memory.
///
impl Handler<FindLeastWorkerUsage> for Server {
    type Result = ResponseActFuture<Self, Option<Worker>>;
    fn handle(&mut self, msg: FindLeastWorkerUsage, _ctx: &mut Context<Self>) -> Self::Result {
        let worker_manager = self.worker_manager.clone();
        Box::pin(
            async move {
                if msg.is_consumer {
                    worker_manager
                        .as_ref()
                        .read()
                        .await
                        .find_least_cworker(msg.min_consumer_numbers)
                        .await
                } else {
                    worker_manager
                        .as_ref()
                        .read()
                        .await
                        .find_least_pworker(msg.min_consumer_numbers)
                        .await
                }
            }
            .into_actor(self)
            .map(|worker, _act, _ctx| worker),
        )
    }
}

///
/// This hanlder is used to create mediasoup worker.
///
impl Handler<CreateWorker> for Server {
    type Result = ResponseActFuture<Self, Worker>;
    fn handle(&mut self, msg: CreateWorker, _ctx: &mut Context<Self>) -> Self::Result {
        let worker_manager = self.worker_manager.clone();
        Box::pin(
            async move {
                let worker;
                {
                    worker = worker_manager
                        .as_ref()
                        .read()
                        .await
                        .create_worker(msg.is_consumer)
                        .await;
                }
                if msg.is_consumer {
                    worker_manager
                        .as_ref()
                        .write()
                        .await
                        .insert_to_consumers(worker.clone())
                } else {
                    worker_manager
                        .as_ref()
                        .write()
                        .await
                        .insert_to_producers(worker.clone())
                }
            }
            .into_actor(self)
            .map(move |worker, _act, ctx| {
                let address = ctx.address().clone();
                let worker_id = worker.id();
                worker
                    .on_close(move || {
                        address.do_send(RemoveWorker {
                            worker_id,
                            is_consumer: msg.is_consumer,
                        });
                    })
                    .detach();
                if !msg.is_consumer {
                    for room_addr in _act.rooms.values() {
                        let worker_manager = _act.worker_manager.clone();
                        let addr = room_addr.clone();
                        actix::spawn(async move {
                            let wk = worker_manager.as_ref().read().await;
                            let router = wk.create_router(false).await;
                            addr.do_send(InternalMessage::SaveRouter(SaveRouter {
                                is_consumer: false,
                                is_current: true,
                                router,
                            }));
                        });
                    }
                }
                worker
            }),
        )
    }
}
///
/// This handler is used to create mediasoup router.
///
impl Handler<CreateRouter> for Server {
    type Result = ResponseActFuture<Self, Router>;
    fn handle(&mut self, msg: CreateRouter, _ctx: &mut Context<Self>) -> Self::Result {
        let worker_manager = self.worker_manager.clone();
        Box::pin(
            async move {
                if msg.worker.is_some() {
                    if msg.is_consumer {
                        worker_manager
                            .as_ref()
                            .read()
                            .await
                            .create_consumer_router(msg.worker.unwrap())
                            .await
                    } else {
                        worker_manager
                            .as_ref()
                            .read()
                            .await
                            .create_producer_router(msg.worker.unwrap())
                            .await
                    }
                } else {
                    worker_manager
                        .as_ref()
                        .read()
                        .await
                        .create_router(msg.is_consumer)
                        .await
                }
            }
            .into_actor(self)
            .map(|router, _act, _ctx| router),
        )
    }
}
