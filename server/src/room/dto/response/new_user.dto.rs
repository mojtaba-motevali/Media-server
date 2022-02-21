use crate::user::UserDto;
use actix::prelude::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct NewUserResponse {
    pub user: UserDto,
}

impl Clone for NewUserResponse {
    fn clone(&self) -> NewUserResponse {
        NewUserResponse {
            user: self.user.clone(),
        }
    }
}
