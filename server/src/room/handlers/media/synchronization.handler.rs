use crate::media::app_data::media_service::MediaAppData;
use crate::room::dto::internal::SynchronizeProducers;
use crate::room::dto::internal::{InternalMessage, SavePipeProducer};
use crate::room::Room;
use actix::prelude::*;
use mediasoup::producer::ProducerId;
use mediasoup::router::Router;
use std::collections::HashMap;
use std::ops::Deref;
use tracing::info;

struct PipeStruct {
    consumer_router: Router,
    producer_id: ProducerId,
}
///
/// This structure is used to Synchronize producers that exist in different workers.
///
impl Handler<SynchronizeProducers> for Room {
    type Result = ResponseActFuture<Self, Result<(), ()>>;
    fn handle(&mut self, message: SynchronizeProducers, ctx: &mut Context<Self>) -> Self::Result {
        let router_producers = self.router_producers.clone();
        let users = self.users.clone();
        let producer_routers = self.producer_routers.clone();
        let address = ctx.address();
        let msg = message.clone();
        Box::pin(
            async move {
                let mut vec_structs: Vec<PipeStruct> = Vec::new();
                let mut futures = HashMap::new();
                for user in users.values() {
                    let producers = { user.as_ref().read().await.get_producers() };
                    for producer in producers {
                        let consumer_router = msg.router.clone();
                        let id = producer.id();
                        if let Some(vec_prod) = router_producers.get(&consumer_router.id()) {
                            let mut found = false;
                            for prod in vec_prod {
                                if *prod == id {
                                    found = true;
                                    break;
                                }
                            }
                            if found {
                                continue;
                            }
                        }
                        let producer_router_id = producer
                            .app_data()
                            .deref()
                            .clone()
                            .downcast::<MediaAppData>()
                            .unwrap()
                            .router_id;
                        let p_router = producer_routers.get(&producer_router_id).unwrap().clone();
                        futures.insert(
                            id,
                            Room::pipe_to_router(p_router, consumer_router.clone(), id),
                        );
                        vec_structs.push(PipeStruct {
                            producer_id: id,
                            consumer_router,
                        });
                    }
                }
                // Some async computation
                for pipe_info in vec_structs {
                    let router_id = pipe_info.consumer_router.id();
                    let producer_id = pipe_info.producer_id;
                    futures.remove(&pipe_info.producer_id).unwrap().await;
                    address.do_send(InternalMessage::SavePipeProducer(SavePipeProducer {
                        router_id,
                        producer_id,
                    }));
                }
            }
            .into_actor(self)
            .map(|_res, act, _ctx| {
                // sometimes we need to insert consumer router here
                if act.consumer_routers.get(&message.router.id()).is_none() {
                    act.current_consumer_id = message.router.id();
                    act.consumer_routers
                        .insert(message.router.id(), message.router);
                }
                info!("here is map and last synchronization step");
                Ok(())
            }),
        )
    }
}
