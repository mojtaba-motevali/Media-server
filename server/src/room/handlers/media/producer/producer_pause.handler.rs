use crate::room::dto::producer_pause_request::ProducerPauseRequest;
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use actix::prelude::*;
use tracing::error;
///
/// This handler is used to serve user's request and pause user's producer stream.
///
impl Handler<ProducerPauseRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ProducerPauseRequest, ctx: &mut Context<Self>) {
        if let Some(my_user) = self.users.get(&msg.user_id) {
            let address = ctx.address();
            let user_lock = my_user.clone();
            actix::spawn(async move {
                let user = user_lock.as_ref().read().await;
                match user.pause_producer(msg.id).await {
                    Ok(()) => {}
                    Err(error) => {
                        error!("Producer: {}", error);
                        address.do_send(DisconnectMessage {
                            id: user.id,
                            send_to_client: true,
                            send_to_room: false,
                            message: error.to_string(),
                        });
                    }
                }
            });
        }
    }
}
