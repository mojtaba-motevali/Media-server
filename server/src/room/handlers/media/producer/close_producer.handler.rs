use crate::room::dto::producer_close_request::ProducerCloseRequest;
use crate::room::Room;
use actix::prelude::*;
///
/// This handler is used to close producer.
/// typically this is used when user stops sending video/audio stream.
///
impl Handler<ProducerCloseRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ProducerCloseRequest, _ctx: &mut Context<Self>) -> Self::Result {
        if let Some(user) = self.users.get(&msg.user_id) {
            let user_lock = user.clone();
            actix::spawn(async move {
                let mut user = user_lock.as_ref().write().await;
                user.remove_producer(msg.id);
            });
        }
    }
}
