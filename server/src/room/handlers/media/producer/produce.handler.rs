use crate::media::app_data::media_service::MediaAppData;
use crate::media::media_struct::DataProducer;
use crate::room::dto::args::produce_dto::ProduceArgsDto;
use crate::room::dto::internal::{InternalMessage, NewProducer, ProduceScale, SaveProducer};
use crate::room::dto::produce_request::ProduceRequest;
use crate::room::dto::produced_response::{AudioProducedResponse, VideoProducedResponse};
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use actix::prelude::*;
use mediasoup::rtp_parameters::MediaKind;
use std::ops::Deref;
///
/// This handler is used to create new producer ordered by user ( when user opens camera or microphone in client. )
///
impl Handler<ProduceRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ProduceRequest, ctx: &mut Context<Self>) {
        if let Some(_user) = self.users.get(&msg.user_id) {
            let user = _user.clone();
            let address = ctx.address();
            actix::spawn(async move {
                let options = ProduceArgsDto {
                    rtp_parameters: msg.rtp_parameters,
                    kind: msg.kind,
                    user_id: msg.user_id,
                    service_type: msg.service_type,
                };
                let mut inner_user = user.as_ref().write().await;

                match inner_user.create_producer(options).await {
                    Ok(producer) => {
                        address
                            .send(ProduceScale {
                                skip_id: msg.user_id,
                                producer: producer.clone(),
                            })
                            .await
                            .unwrap()
                            .unwrap();
                        if producer.kind() == MediaKind::Audio {
                            inner_user
                                .ws_actor_addr
                                .do_send(AudioProducedResponse { id: producer.id() });
                        } else {
                            inner_user
                                .ws_actor_addr
                                .do_send(VideoProducedResponse { id: producer.id() });
                        }
                        address.do_send(InternalMessage::SaveProducer(SaveProducer {
                            producer: producer.clone(),
                            user_id: inner_user.id,
                        }));
                        let app_data = producer
                            .app_data()
                            .deref()
                            .clone()
                            .downcast::<MediaAppData>()
                            .unwrap()
                            .deref()
                            .clone();
                        address.do_send(NewProducer {
                            data_producer: DataProducer {
                                id: producer.id(),
                                app_data: app_data,
                                user_id: inner_user.id,
                            },
                        });
                    }
                    Err(error) => {
                        eprintln!("Producer: {}", error);
                        address.do_send(DisconnectMessage {
                            id: inner_user.id,
                            send_to_client: true,
                            send_to_room: false,
                            message: error.to_string(),
                        });
                    }
                };
            });
        }
    }
}
