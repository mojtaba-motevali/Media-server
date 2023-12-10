use crate::media::app_data::media_service::MediaAppData;
use crate::room::dto::internal::{
    InternalMessage, RemoveProducer, SaveAudioLevelObserver, SaveConsumer, SavePipeProducer,
    SaveProducer, SaveRouter,
};
use crate::room::message::RoomActiveSpeakerDetector;
use crate::room::Room;
use actix::prelude::*;
use mediasoup::audio_level_observer::{AudioLevelObserverOptions, AudioLevelObserverVolume};
use std::env;
use std::iter::Iterator;
use std::ops::Deref;
use tracing::info;
///
/// This handler is used by to signal asynchronous operations.
///
impl Handler<InternalMessage> for Room {
    type Result = ();

    fn handle(&mut self, msg: InternalMessage, ctx: &mut Context<Self>) -> Self::Result {
        // check if user exists in room
        match msg {
            InternalMessage::SaveAudioLevelObserver(SaveAudioLevelObserver {
                router_id,
                audio_level_observer,
            }) => {
                audio_level_observer
                    .on_silence(|| {
                        info!("It's silenceeeeee");
                    })
                    .detach();
                let address = ctx.address();
                audio_level_observer
                    .on_volumes(move |voloumes: &[AudioLevelObserverVolume]| {
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
                                info!("Error While downcasting data app");
                            }
                        }
                    })
                    .detach();
                self.audio_level_observers
                    .insert(router_id, audio_level_observer);
            }
            InternalMessage::SaveRouter(SaveRouter {
                router,
                is_consumer,
                is_current,
            }) => {
                if is_consumer {
                    if is_current {
                        self.current_consumer_id = router.id();
                    }
                    self.consumer_routers.insert(router.id(), router);
                } else {
                    if is_current {
                        self.current_producer_id = router.id();
                    }
                    let p_router = router.clone();
                    let address = ctx.address().clone();
                    actix::spawn(async move {
                        let mut options = AudioLevelObserverOptions::default();
                        options.interval = env::var("AUDIO_INTERVAL_MSC").unwrap().parse().unwrap();
                        let audio_level_observer =
                            match p_router.create_audio_level_observer(options).await {
                                Ok(obser) => obser,
                                Err(err) => {
                                    let message = "error while creating audioLevelObserver "
                                        .to_string()
                                        + &err.to_string();
                                    info!("{}", message);
                                    return;
                                }
                            };
                        address.do_send(InternalMessage::SaveAudioLevelObserver(
                            SaveAudioLevelObserver {
                                router_id: p_router.id(),
                                audio_level_observer,
                            },
                        ));
                    });

                    self.producer_routers.insert(router.id(), router);
                }
            }
            InternalMessage::SaveProducer(SaveProducer { producer, user_id }) => {
                if let Some(_user) = self.users.get(&user_id) {
                    let user = _user.clone();
                    actix::spawn(async move {
                        user.as_ref().write().await.insert_producer(producer);
                    });
                }
            }
            InternalMessage::SaveConsumer(SaveConsumer { consumer, user_id }) => {
                if let Some(_user) = self.users.get(&user_id) {
                    let user = _user.clone();
                    actix::spawn(async move {
                        user.as_ref().write().await.insert_consumer(consumer);
                    });
                }
            }
            // InternalMessage::RemoveConsumer( RemoveConsumer {consumer_id,user_id} ) => {
            //     if let Some(_user) = self.users.get(&user_id) {
            //         let user = _user.clone();
            //         actix::spawn(async move {
            //             user.as_ref().write().await.remove_consumer(consumer_id);
            //         });
            //     }
            // },
            InternalMessage::RemoveProducer(RemoveProducer {
                producer_id,
                user_id,
            }) => {
                if let Some(_user) = self.users.get(&user_id) {
                    let user = _user.clone();
                    actix::spawn(async move {
                        user.as_ref().write().await.remove_producer(producer_id);
                    });
                }
            }
            InternalMessage::SavePipeProducer(SavePipeProducer {
                producer_id,
                router_id,
            }) => {
                if let Some(prod_vec) = self.router_producers.get_mut(&router_id) {
                    let mut iter = prod_vec.into_iter();
                    if iter.find(|&&mut x| x == producer_id).is_none() {
                        prod_vec.push(producer_id);
                    }
                } else {
                    let mut prod_vec = vec![];
                    prod_vec.push(producer_id);
                    self.router_producers.insert(router_id, prod_vec);
                }
            }
        }
    }
}
