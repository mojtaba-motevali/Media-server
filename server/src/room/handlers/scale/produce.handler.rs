use crate::media::app_data::media_service::MediaAppData;
use crate::media::app_data::router::ConsumerRouterAppData;
use crate::room::dto::internal::{InternalMessage, ProduceScale, SavePipeProducer, SaveRouter};
use crate::room::Room;
use crate::server::message::{CanHandleConsumer, CreateRouter, CreateWorker};
use crate::server::Server;
use crate::user::User;
use actix::prelude::*;
use async_rwlock::RwLock;
use mediasoup::producer::Producer;
use mediasoup::router::Router;
use mediasoup::worker::Worker;
use std::env;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

fn slice_creator(users: &mut Vec<Rc<RwLock<User>>>, total_number: usize) -> Vec<Rc<RwLock<User>>> {
    let mut vec_users = vec![];
    vec_users.reserve(total_number);
    if users.len() > total_number {
        for _ in 0..total_number {
            vec_users.push(users.pop().unwrap());
        }
        return vec_users;
    } else {
        for _ in 0..users.len() {
            vec_users.push(users.pop().unwrap());
        }
        return vec_users;
    }
}
async fn create_workers(server_addr: &Addr<Server>, number_of_workers: usize) -> Vec<Worker> {
    let mut workers = vec![];
    let mut futures = vec![];
    for _ in 0..number_of_workers {
        let worker = server_addr.send(CreateWorker { is_consumer: true });
        futures.push(worker);
    }
    let vec_length = futures.len();
    for _ in 0..vec_length {
        workers.push(futures.pop().unwrap().await.unwrap());
    }
    workers
}
struct CreateNewRouter {
    server_addr: Addr<Server>,
    p_router: Router,
    workers: Vec<Worker>,
    producer: Producer,
}
async fn create_routers(
    CreateNewRouter {
        producer,
        p_router,
        mut workers,
        server_addr,
    }: CreateNewRouter,
) -> Vec<Router> {
    let mut futures = vec![];
    let mut pipe_futures = vec![];
    let mut routers = vec![];
    let mut vec_length = workers.len();
    for _ in 0..vec_length {
        let future = server_addr.send(CreateRouter {
            is_consumer: true,
            worker: Some(workers.pop().unwrap()),
        });
        futures.push(future);
    }
    vec_length = futures.len();
    for _ in 0..vec_length {
        let c_router = futures.pop().unwrap().await.unwrap();
        pipe_futures.push(Room::pipe_to_router(
            p_router.clone(),
            c_router.clone(),
            producer.id(),
        ));
        routers.push(c_router);
    }
    vec_length = pipe_futures.len();
    for _ in 0..vec_length {
        pipe_futures.pop().unwrap().await;
    }
    routers
}
///
/// This handler is used to scale application when a new producer is being created if needed.
///
impl Handler<ProduceScale> for Room {
    type Result = ResponseActFuture<Self, Result<(), ()>>;

    fn handle(&mut self, msg: ProduceScale, ctx: &mut Context<Self>) -> Self::Result {
        let address = ctx.address();
        let consumer_numbers = self.users.len() - 1;
        let router = self
            .consumer_routers
            .get(&self.current_consumer_id)
            .unwrap()
            .clone();
        let server_addr = self.server_addr.clone();
        let mut users = {
            let mut vec = vec![];
            for user in self.users.iter() {
                if *(user.0) != msg.skip_id {
                    vec.push(user.1.clone());
                }
            }
            vec
        };
        let producer_routers = self.producer_routers.clone();
        let producer = msg.producer;
        Box::pin(
            async move {
                let temp = router
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<RwLock<ConsumerRouterAppData>>()
                    .unwrap();
                let app_data = {
                    let rw_app_data: async_rwlock::RwLockReadGuard<
                        crate::media::app_data::router::ConsumerRouterAppData,
                    > = temp.read().await;
                    let app_data = rw_app_data.clone();
                    drop(rw_app_data);
                    app_data
                };
                let can_handle = server_addr
                    .send(CanHandleConsumer {
                        number_consumers: consumer_numbers,
                        is_consumer: true,
                        worker_pid: app_data.worker_pid,
                    })
                    .await
                    .unwrap();
                let router_id = producer
                    .app_data()
                    .deref()
                    .clone()
                    .downcast::<MediaAppData>()
                    .unwrap()
                    .as_ref()
                    .router_id;
                let p_router = producer_routers.get(&router_id).unwrap();
                if !can_handle {
                    let max_number_of_consumers: usize =
                        env::var("WORKER_CONSUMER_MAX").unwrap().parse().unwrap();
                    let mut number_of_workers = consumer_numbers / max_number_of_consumers;
                    if number_of_workers < 1 {
                        number_of_workers = 1;
                    }
                    let workers = create_workers(&server_addr, number_of_workers).await;
                    let routers = create_routers(CreateNewRouter {
                        server_addr,
                        producer,
                        workers,
                        p_router: p_router.clone(),
                    })
                    .await;
                    if routers.len() == 1 {
                        address.do_send(InternalMessage::SaveRouter(SaveRouter {
                            is_consumer: true,
                            is_current: true,
                            router: routers[0].clone(),
                        }));
                        Room::create_missing_transports(&users, routers[0].clone(), true).await;
                    } else {
                        let mut i = 0;
                        while i < routers.len() && users.len() > 0 {
                            let partial_users = slice_creator(&mut users, max_number_of_consumers);
                            address.do_send(InternalMessage::SaveRouter(SaveRouter {
                                is_consumer: true,
                                is_current: i == (routers.len() - 1),
                                router: routers[i].clone(),
                            }));
                            Room::create_missing_transports(
                                &partial_users,
                                routers[i].clone(),
                                true,
                            )
                            .await;
                            i += 1;
                        }
                    }
                } else {
                    let _ =
                        Room::pipe_to_router(p_router.clone(), router.clone(), producer.id()).await;
                    address.do_send(InternalMessage::SavePipeProducer(SavePipeProducer {
                        producer_id: producer.id(),
                        router_id: router.id(),
                    }));
                    Room::create_missing_transports(&users, router.clone(), true).await;
                }
                Ok(())
            }
            .into_actor(self),
        )
    }
}
