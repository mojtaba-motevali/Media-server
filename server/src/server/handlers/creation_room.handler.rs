use crate::server::Server;
use actix::prelude::*;

use crate::server::message::AsyncRoomCreation;

///
/// This handler is used to store created room and it's arbiter.
///
impl Handler<AsyncRoomCreation> for Server {
    type Result = ();
    fn handle(&mut self, msg: AsyncRoomCreation, _: &mut Context<Self>) -> Self::Result {
        self.arbiters.insert(msg.room_id.clone(), msg.arb);
        self.rooms.insert(msg.room_id, msg.room_addr);
    }
}
