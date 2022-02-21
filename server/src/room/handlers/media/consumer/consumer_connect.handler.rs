use crate::room::dto::c_c_transport_request::ConnectConsumerTransportRequest;
use crate::room::dto::c_c_transport_response::ConnectedConsumerTransportResponse;
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use actix::prelude::*;
///
/// This handler is used to serve user's request and connect consumer transports ( one in client and one in server).
///
impl Handler<ConnectConsumerTransportRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ConnectConsumerTransportRequest, ctx: &mut Context<Self>) {
        if let Some(my_user) = self.users.get(&msg.user_id) {
            let user_lock = my_user.clone();
            let address = ctx.address();
            actix::spawn(async move {
                let user = user_lock.as_ref().read().await;
                match user.connect_consumer_transport(msg).await {
                    Ok(()) => {
                        user.ws_actor_addr
                            .do_send(ConnectedConsumerTransportResponse {});
                    }
                    Err(error) => {
                        eprintln!("connect_consumer_transport handler: {}", error);
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
