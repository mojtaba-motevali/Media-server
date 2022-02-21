use actix::prelude::*;
use std::ops::Deref;
use crate::room::dto::rtp_capability_settings_request::{RtpCapabilitiesSettings};
use crate::room::Room;
use crate::room::dto::consumed_response::ConsumedResponse;
use crate::session::room_actor_msg::{DisconnectMessage};
use crate::room::dto::consumer_close_request::ConsumerCloseRequest;
use crate::room::dto::args::consume_dto::ConsumeArgsDto;
use crate::media::app_data::media_service::MediaAppData;
use crate::room::dto::internal::{SaveConsumer,InternalMessage};
use crate::room::dto::internal::SynchronizeProducers;
use mediasoup::router::Router;
///
/// This handler is used to set rtp capabilities sent by user from client and also creates 
/// consumers from each producer that exist in the room. 
/// 
impl Handler<RtpCapabilitiesSettings> for Room {
    type Result = ();
    fn handle(&mut self,msg:RtpCapabilitiesSettings,ctx:&mut Context<Self>) -> Self::Result {
            let users = self.users.clone();
            let future_map = async  move {
                Room::get_producer_users(&users).await
                }.into_actor(self).map(|producers,_act,ctx|{

                if let Some(user) = _act.users.get(&msg.user_id) {
                    let _user = user.clone();
                    let producer_router = _act.producer_routers.get(&_act.current_producer_id).unwrap().clone();
                    let consumer_router:Router = _act.consumer_routers.get(&_act.current_consumer_id).unwrap().clone();
                    let _address = ctx.address().clone();
                    actix::spawn( async move {
                        let cloned_user = _user.clone();
                        {
                            cloned_user.as_ref().write().await.self_rtp_capabilities.replace(msg.rtp_capabilities.clone());
                        }
                        let fut1 = _address.send(SynchronizeProducers{router:consumer_router.clone()});
                        let fut2 = Room::create_webrtc_transport(cloned_user.clone(), producer_router.clone(),false);
                        let fut3 = Room::create_webrtc_transport(cloned_user.clone(), consumer_router.clone(),true);
                        {
                            let _ = fut1.await;
                        }
                        fut2.await;
                        fut3.await;
                        for producer in  producers {
                            let user = cloned_user.as_ref().read().await;
                            let id = user.id;
                            let address = _address.clone();
                            let options = ConsumeArgsDto{ 
                                producer_id:producer.id,
                                user_id:id,
                                service_type:producer.app_data.service_type
                            };
                            let ws_actor_addr = user.ws_actor_addr.clone();
                            match user.create_consumer(options).await {
                                Ok(consumer) => { 
                                    println!(" creating consumers while rtp_capability {:?}",consumer.id());
                                    let consumer_id = consumer.id();

                                    ws_actor_addr.do_send(ConsumedResponse {
                                        kind:consumer.kind(),
                                        id:consumer_id,
                                        producer_id:producer.id,
                                        rtp_parameters:consumer.rtp_parameters().clone(),
                                        user_id:producer.user_id,
                                        app_data:consumer.app_data().deref().clone().downcast::<MediaAppData>().unwrap().deref().clone()
                                    });
                                    let _address = address.clone();
                                    consumer.on_producer_close( Box::new(move || {
                                        println!("producer of {:?} in rtp_setting_capa() consumer closed", consumer_id);
                                        _address.do_send(ConsumerCloseRequest{user_id:id,id:consumer_id,producer_user_id:producer.app_data.user_id});
                                    })).detach();
                                    address.do_send(
                                        InternalMessage::SaveConsumer(
                                            SaveConsumer{ consumer,user_id:user.id }
                                        )  
                                    );
                                },
                                Err(error) => {
                                    println!(" creating consumers while getting rtp_capabilities {:?}",error);
                                    address.do_send(DisconnectMessage{id:id,send_to_client:true,send_to_room:false,message:error.to_string()});
                                }
                            }
                            }
                            
                    });
                }
            });
            ctx.wait(future_map);
    }
}
