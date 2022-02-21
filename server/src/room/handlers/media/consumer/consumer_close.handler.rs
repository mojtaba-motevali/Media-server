use crate::room::dto::consumer_close_request::ConsumerCloseRequest;
use crate::room::dto::consumer_close_response::ConsumerCloseResponse;
use crate::room::Room;
use actix::prelude::*;
///
/// This handler is used to serve user's request to close consumer stream.
///
impl Handler<ConsumerCloseRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ConsumerCloseRequest, _ctx: &mut Context<Self>) {
        if let Some(my_user) = self.users.get(&msg.user_id) {
            let user_lock = my_user.clone();
            actix::spawn(async move {
                let mut user = user_lock.as_ref().write().await;
                user.remove_consumer(msg.id);
                user.ws_actor_addr.do_send(ConsumerCloseResponse {
                    user_id: msg.producer_user_id,
                    id: msg.id,
                });
            });
        }
    }
}
