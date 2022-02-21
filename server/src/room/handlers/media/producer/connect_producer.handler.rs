use crate::room::dto::c_p_transport_request::ConnectProducerTransportRequest;
use crate::room::dto::c_p_transport_response::ConnectedProducerTransportResponse;
use crate::room::Room;
use crate::session::room_actor_msg::DisconnectMessage;
use actix::prelude::*;
///
/// This handler is used to connect producer transports created in both client/server.
///
impl Handler<ConnectProducerTransportRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: ConnectProducerTransportRequest, ctx: &mut Context<Self>) {
        if let Some(my_user) = self.users.get(&msg.user_id) {
            let user_lock = my_user.clone();
            let address = ctx.address();
            actix::spawn(async move {
                let user = user_lock.as_ref().read().await;
                match user.connect_producer_transport(msg).await {
                    Ok(()) => {
                        user.ws_actor_addr
                            .do_send(ConnectedProducerTransportResponse {});
                    }
                    Err(error) => {
                        eprintln!("connect_producer_transport handler: {}", error);
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
