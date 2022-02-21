use crate::media::app_data::router::{ConsumerRouterAppData, ProducerRouterAppData};
use crate::room::dto::internal::EnteranceScale;
use crate::room::dto::internal::SynchronizeProducers;
use crate::room::Room;
use crate::server::message::{CanHandleConsumer, CreateRouter, CreateWorker, FindLeastWorkerUsage};
use actix::prelude::*;
use async_rwlock::RwLock;
use mediasoup::router::RouterId;
use std::ops::Deref;
///
/// This handler is used to scale application when users join if needed.
///
impl Handler<EnteranceScale> for Room {
    type Result = ();

    fn handle(&mut self, _: EnteranceScale, ctx: &mut Context<Self>) -> Self::Result {
        // check if user exists in room
        let server_addr = self.server_addr.clone();
        let consumer_routers = self.consumer_routers.clone();
        let addr = ctx.address().clone();
        let producer_routers = self.producer_routers.clone();
        let current_consumer_id = self.current_consumer_id.clone();
        actix::spawn(async move {
            let mut number_consumers = 0;
            for router in producer_routers.values() {
                let app_lock = router
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<RwLock<ProducerRouterAppData>>()
                    .unwrap();
                let app_data = app_lock.read().await;
                number_consumers += app_data.number_of_consumers;
            }
            let consumer_router = consumer_routers.get(&current_consumer_id).unwrap();
            let app_data: ConsumerRouterAppData = {
                consumer_router
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<RwLock<ConsumerRouterAppData>>()
                    .unwrap()
                    .as_ref()
                    .read()
                    .await
                    .clone()
            };
            let can_handle: bool = server_addr
                .send(CanHandleConsumer {
                    number_consumers,
                    worker_pid: app_data.worker_pid.clone(),
                    is_consumer: true,
                })
                .await
                .unwrap();
            // if current worker can't handle this amount of consumers
            if !can_handle {
                println!("cant handle");
                // find least worker usage
                let worker = server_addr
                    .send(FindLeastWorkerUsage {
                        min_consumer_numbers: number_consumers,
                        is_consumer: true,
                    })
                    .await
                    .unwrap();
                if worker.is_some() {
                    let mut router_id: Option<RouterId> = None;
                    // if a worker has been found , then find whether current room as a router in it or not
                    let worker_id = worker.clone().unwrap().id();
                    for router in consumer_routers.values() {
                        let wk_id = worker_id.clone();
                        let router_app_data = router
                            .app_data()
                            .deref()
                            .clone()
                            .downcast::<RwLock<ConsumerRouterAppData>>()
                            .unwrap()
                            .read()
                            .await
                            .clone();
                        if router_app_data.worker_pid == wk_id {
                            router_id.replace(router.id());
                            break;
                        }
                    }
                    // if router has been found in the worker
                    if router_id.is_some() {
                        let router = consumer_routers.get(&(router_id.unwrap())).unwrap();
                        // sync producers with it
                        addr.do_send(SynchronizeProducers {
                            router: router.clone(),
                        });
                    } else {
                        // if no router found in that worker, create new router in the worker
                        let router = server_addr
                            .send(CreateRouter {
                                worker,
                                is_consumer: true,
                            })
                            .await
                            .unwrap();
                        // then sync producers with it
                        addr.do_send(SynchronizeProducers {
                            router: router.clone(),
                        });
                    }
                }
                // no min worker found
                else {
                    actix::spawn(async move {
                        // request server to create worker for us
                        let worker = server_addr
                            .send(CreateWorker { is_consumer: true })
                            .await
                            .unwrap();
                        // request server to create consumer router in the worker
                        let router = server_addr
                            .send(CreateRouter {
                                worker: Some(worker),
                                is_consumer: true,
                            })
                            .await
                            .unwrap();
                        // sync producers with the router
                        addr.do_send(SynchronizeProducers {
                            router: router.clone(),
                        });
                    });
                }
            }
        });
    }
}
