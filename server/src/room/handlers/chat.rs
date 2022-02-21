use crate::chat::{BroadcastMessage, PrivateMessage};
use crate::room::dto::broadcast_message_request::BroadcastMessageRequest;
use crate::room::dto::private_message_request::PrivateMessageRequest;
use crate::room::dto::private_message_response::PrivateMessageResponse;
use crate::room::Room;
use actix::prelude::*;

///
/// This handler is used by user to send private message to another user.
///
impl Handler<PrivateMessageRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: PrivateMessageRequest, ctx: &mut Context<Self>) {
        if let Some(_user) = self.users.get(&(msg.receiver_id)) {
            let user = _user.clone();
            async move {
                let message =
                    PrivateMessage::new(msg.sender_id, msg.receiver_id, msg.message, msg.timestamp);
                user.as_ref()
                    .read()
                    .await
                    .ws_actor_addr
                    .do_send(PrivateMessageResponse {
                        id: message.id,
                        message: message.message.clone(),
                        receiver_id: message.receiver_id,
                        sender_id: message.sender_id,
                        timestamp: message.timestamp,
                    });
                message
            }
            .into_actor(self)
            .map(|message, _act, _ctx| {
                _act.private_messages.push(message);
            })
            .wait(ctx);
        }
    }
}
///
/// This handler is used to broadcast a message sent from user.
///
impl Handler<BroadcastMessageRequest> for Room {
    type Result = ();
    fn handle(&mut self, msg: BroadcastMessageRequest, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(_user) = self.users.get(&(msg.sender_id)) {
            let users = self.users.clone();
            let user = _user.clone();
            async move {
                let name = { user.as_ref().read().await.name.clone() };
                let message =
                    BroadcastMessage::new(msg.sender_id, msg.message, name, msg.timestamp);
                Room::send_message(&users, message.clone()).await;
                message
            }
            .into_actor(self)
            .map(|message: BroadcastMessage, _act, _ctx| {
                _act.messages.push(message.clone());
            })
            .wait(ctx);
        }
    }
}
