use crate::user::UserDto;
use actix::prelude::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct NewUserRequest {
    pub user: UserDto,
}
