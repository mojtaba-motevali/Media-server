use actix::*;

#[derive(Message)]
#[rtype(result = "()")]
pub enum InternalErrorMessage {
    UnAuthorized(String),
    // BadRequest(String),
    ServerFault(String),
    InvalidJson(String),
    NotFound(String),
}
