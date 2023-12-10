use crate::media::app_data::app_traits::AppDataOperation;
use crate::server::message::{
    AddConsumer, AddProducer, CreateWorker, RemovedConsumer, RemovedProducer,
};
use crate::server::Server;
use actix::prelude::*;
use tracing::info;

///
/// This hanlder is used to decrease number of consumers.
///
impl Handler<RemovedConsumer> for Server {
    type Result = ();
    fn handle(&mut self, msg: RemovedConsumer, _ctx: &mut Context<Self>) -> Self::Result {
        let rapp_data = msg.consumer_router_app_data;
        let wapp_data = msg.consumer_worker_app_data;
        actix::spawn(async move {
            {
                let mut data = wapp_data.as_ref().write().await;
                data.subtract(1);
                info!("data_worker_consumer:{:?}", data.get());
            }
            {
                let mut data = rapp_data.as_ref().write().await;
                data.subtract(1);
                info!("data_router_consumer:{:?}", data.get());
            }
        });
    }
}
///
/// This hanlder is used to increase number of consumers.
///
impl Handler<AddConsumer> for Server {
    type Result = ();
    fn handle(&mut self, msg: AddConsumer, _ctx: &mut Context<Self>) -> Self::Result {
        let rapp_data = msg.consumer_router_app_data;
        let wapp_data = msg.consumer_worker_app_data;
        actix::spawn(async move {
            {
                let mut data = rapp_data.as_ref().write().await;
                data.add(1);
                info!("data_router_consumer:{:?}", data.get());
            }
            {
                let mut data = wapp_data.as_ref().write().await;
                data.add(1);
                info!("data_worker_consumer:{:?}", data.get());
            }
        });
    }
}
///
/// This hanlder is used to decrease number of producers.
///
impl Handler<RemovedProducer> for Server {
    type Result = ();
    fn handle(&mut self, msg: RemovedProducer, _ctx: &mut Context<Self>) -> Self::Result {
        let rapp_data = msg.producer_router_app_data;
        let wapp_data = msg.producer_worker_app_data;
        actix::spawn(async move {
            {
                let mut data = rapp_data.as_ref().write().await;
                data.subtract(1);
                info!("number of router_producer:{:?}", data.get());
            }
            {
                let mut data = wapp_data.as_ref().write().await;
                data.subtract(1);
                info!("number of worker_producer:{:?}", data.get());
            }
        });
    }
}
///
/// This hanlder is used to increase number of producers.
///
impl Handler<AddProducer> for Server {
    type Result = ();
    fn handle(&mut self, msg: AddProducer, _ctx: &mut Context<Self>) -> Self::Result {
        let rapp_data = msg.producer_router_app_data;
        let wapp_data = msg.producer_worker_app_data;
        let threashold = msg.threashold;
        let address = _ctx.address().clone();
        actix::spawn(async move {
            {
                let data = wapp_data.as_ref().read().await;
                if data.get() + 1 >= threashold {
                    address.do_send(CreateWorker { is_consumer: false });
                }
            }
            {
                let mut data = rapp_data.as_ref().write().await;
                data.add(1);
                info!("number of data_producer: {:?}", data.get());
            }
            {
                let mut data = wapp_data.as_ref().write().await;
                data.add(1);
                info!("number of worker_producer:{:?}", data.get());
            }
        });
    }
}
