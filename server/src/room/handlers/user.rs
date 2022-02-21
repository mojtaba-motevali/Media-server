use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::room::dto::internal::EnteranceScale;
use crate::room::dto::join_room_response::JoinRoomActorResponse;
use crate::room::dto::new_user_response::NewUserResponse;
use crate::room::Room;
use crate::server::message::RoomUserEntranceMessage;
use crate::session::room_actor_msg::{DisconnectMessage, PeerDisconnectMessage};
use crate::user::{User, UserDto};
use actix::prelude::*;
use async_rwlock::RwLock;
use std::rc::Rc;
///
/// This handler is used to join user to a room and disconnects other user's sessions.
///
impl Handler<RoomUserEntranceMessage> for Room {
    type Result = ResponseActFuture<Self, JoinRoomActorResponse>;

    fn handle(&mut self, msg: RoomUserEntranceMessage, ctx: &mut Context<Self>) -> Self::Result {
        let address = ctx.address().clone();
        let already_joined = self.users.remove(&(msg.user.id.unwrap()));
        if already_joined.is_some() {
            actix::spawn(async move {
                let id = { already_joined.unwrap().as_ref().read().await.id };
                address.do_send(DisconnectMessage {
                    id,
                    send_to_client: true,
                    send_to_room: false,
                    message: String::from("You've been disconnected by another session."),
                });
            });
        }
        ctx.notify(EnteranceScale {});
        let user_msg = msg.user.clone();
        self.user_addresses
            .insert(msg.user.id.unwrap(), msg.addr.clone());
        let user = Rc::new(RwLock::new(User::new(
            msg.user.id.unwrap().clone(),
            msg.user.name.unwrap(),
            msg.addr.clone(),
        )));
        self.users.insert(msg.user.id.unwrap(), user.clone());
        let users = self.users.clone();
        Box::pin(
            async move {
                Room::notify_join_user(
                    &users,
                    NewUserResponse {
                        user: user_msg.clone(),
                    },
                    user_msg.id.unwrap(),
                )
                .await;
                let mut _users = vec![];
                for user in users.values() {
                    let _user = user.as_ref().read().await;
                    _users.push(UserDto {
                        id: Some(_user.id),
                        name: Some(_user.name.clone()),
                    });
                }
                _users
            }
            .into_actor(self)
            .map(|res, act, ctx| {
                // let consumer_router = act.consumer_routers.get(&act.current_consumer_id).unwrap().clone();
                // let router_rtp_capabilities = consumer_router.rtp_capabilities().clone();
                let producer_router = act
                    .producer_routers
                    .get(&act.current_producer_id)
                    .unwrap()
                    .clone();
                let router_rtp_capabilities = producer_router.rtp_capabilities().clone();
                let messages = act
                    .messages
                    .iter()
                    .map(|message| BroadcastMessageResponse {
                        id: message.id,
                        sender_id: message.sender_id,
                        user_name: message.user_name.clone(),
                        message: message.message.clone(),
                        timestamp: message.timestamp,
                    })
                    .collect();
                JoinRoomActorResponse {
                    users: res,
                    room_addr: ctx.address(),
                    messages,
                    router_rtp_capabilities,
                }
            }),
        )
    }
}
///
/// This handler is used to disconnect user from room.
///
impl Handler<DisconnectMessage> for Room {
    type Result = ();
    fn handle(&mut self, msg: DisconnectMessage, _ctx: &mut Context<Self>) -> Self::Result {
        println!(
            "removing {:?} from users because {:?} ",
            msg.id, msg.message
        );
        if let Some(disconneced_user) = self.users.remove(&msg.id) {
            let user_addr = self.user_addresses.remove(&msg.id).unwrap();
            let users = self.users.clone();
            actix::spawn(async move {
                {
                    disconneced_user.as_ref().write().await.clean_up().await;
                }
                for user in users.values() {
                    let _user = user.as_ref().read().await;
                    _user
                        .ws_actor_addr
                        .do_send(PeerDisconnectMessage { user_id: msg.id });
                }
                if msg.send_to_client {
                    user_addr.do_send(DisconnectMessage {
                        id: msg.id,
                        send_to_client: true,
                        send_to_room: false,
                        message: msg.message,
                    });
                }
            });
        }
    }
}
