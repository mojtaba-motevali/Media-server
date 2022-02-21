use crate::media::app_data::worker::{ConsumerWorkerAppData, ProducerWorkerAppData};
use crate::room::message::CloseInactiveRoomMessage;
use crate::server::Server;
use actix::prelude::{Context, Handler};

///
/// This handler is used to remove closed room and clean up workers.
///
impl Handler<CloseInactiveRoomMessage> for Server {
    type Result = ();
    fn handle(&mut self, msg: CloseInactiveRoomMessage, _: &mut Context<Self>) -> Self::Result {
        self.rooms.remove(&msg.0);
        self.arbiters.remove(&msg.0).unwrap().stop();
        let worker_manager = self.worker_manager.clone();
        actix::spawn(async move {
            let mut wk = worker_manager.as_ref().write().await;
            wk.clean_workers::<ConsumerWorkerAppData>(true).await;
            wk.clean_workers::<ProducerWorkerAppData>(false).await;
        });
    }
}
