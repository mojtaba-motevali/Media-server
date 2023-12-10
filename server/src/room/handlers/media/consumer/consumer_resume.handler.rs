use crate::room::dto::consumer_resume_request::ConsumerResumeRequest;
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use actix::prelude::*;
use tracing::error;
///
/// This handler is used to serve user's request and resume user's consumer stream.
///
impl Handler<ConsumerResumeRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ConsumerResumeRequest, ctx: &mut Context<Self>) {
        if let Some(my_user) = self.users.get(&msg.user_id) {
            let user_lock = my_user.clone();
            let address = ctx.address();
            actix::spawn(async move {
                let user = user_lock.as_ref().read().await;
                match user.resume_consumer(msg.id).await {
                    Ok(()) => {}
                    Err(error) => {
                        error!("consumer_resume_request handler: {}", error);
                        address.do_send(DisconnectMessage {
                            id: user.id,
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
