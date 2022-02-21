use actix::prelude::*;
use std::ops::Deref;

use crate::media::app_data::media_service::MediaAppData;
use crate::media::app_data::media_service::MediaServiceType;
use crate::media::media_struct::DataProducer;
use crate::room::dto::args::consume_dto::ConsumeArgsDto;
use crate::room::dto::consumed_response::ConsumedResponse;
use crate::room::dto::consumer_close_request::ConsumerCloseRequest;
use crate::room::dto::internal::NewProducer;
use crate::room::dto::internal::{InternalMessage, SaveConsumer};
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use mediasoup::rtp_observer::{RtpObserver, RtpObserverAddProducerOptions};
///
/// This handler is used to create a new consumer from newly created producer for each user.
/// also notifies users with newly created consumers.
///
impl Handler<NewProducer> for Room {
    type Result = ();
    fn handle(&mut self, msg: NewProducer, ctx: &mut Context<Self>) -> Self::Result {
        let DataProducer {
            id,
            user_id,
            app_data,
        } = msg.data_producer;
        let service_type = app_data.service_type.clone();
        let producer_id = id;
        let audio_obs = self
            .audio_level_observers
            .get(&app_data.router_id)
            .unwrap()
            .clone();
        actix::spawn(async move {
            match service_type {
                MediaServiceType::VOICE => {
                    match audio_obs
                        .add_producer(RtpObserverAddProducerOptions::new(producer_id))
                        .await
                    {
                        Ok(()) => {}
                        Err(err) => {
                            println!(" error while adding audio_producer to list {:?}", err);
                        }
                    };
                }
                MediaServiceType::CAMERA => {}
                MediaServiceType::SCREENSHARE => {}
            };
        });
        for user_lock in self.users.values() {
            let user = user_lock.clone();
            let address = ctx.address();
            let _app_data = app_data.clone();
            actix::spawn(async move {
                let _user_id = { user.as_ref().read().await.id };
                if _user_id != user_id {
                    let options = ConsumeArgsDto {
                        producer_id: id,
                        user_id: _user_id,
                        service_type: _app_data.service_type,
                    };
                    let inner_user = user.as_ref().read().await;
                    match inner_user.create_consumer(options).await {
                        Ok(consumer) => {
                            println!("consumer creating from new_producer... {:?}", consumer.id());
                            let consumer_id = consumer.id();
                            inner_user.ws_actor_addr.do_send(ConsumedResponse {
                                kind: consumer.kind(),
                                id: consumer_id,
                                producer_id: id,
                                rtp_parameters: consumer.rtp_parameters().clone(),
                                user_id,
                                app_data: consumer
                                    .app_data()
                                    .deref()
                                    .clone()
                                    .downcast::<MediaAppData>()
                                    .unwrap()
                                    .deref()
                                    .clone(),
                            });

                            let _address = address.clone();
                            consumer.on_producer_close( Box::new(move || {
                                println!("producer of {:?} consumer closed in new_producer.hanlder()", consumer_id);
                                _address.do_send(
                                    ConsumerCloseRequest{user_id:_user_id,id:consumer_id,producer_user_id:_app_data.user_id}
                                );
                            })).detach();
                            address.do_send(InternalMessage::SaveConsumer(SaveConsumer {
                                consumer,
                                user_id: inner_user.id,
                            }));
                        }
                        Err(error) => {
                            println!("new_producer error while creating consumer {:?}", error);
                            address.do_send(DisconnectMessage {
                                id: user_id,
                                send_to_client: true,
                                send_to_room: false,
                                message: error.to_string(),
                            });
                        }
                    }
                }
            });
        }
    }
}
