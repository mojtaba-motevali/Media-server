use crate::user::UserDto;
use actix::prelude::Message;
use serde::Serialize;

#[derive(Message, Serialize, Debug)]
#[rtype(result = "()")]
pub struct UserListResponse {
    pub users: Vec<UserDto>,
}
