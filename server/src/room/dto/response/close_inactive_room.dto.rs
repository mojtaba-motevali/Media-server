use actix::prelude::Message;

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
pub struct CloseInactiveRoomMessage(pub String);
