use crate::user::UserDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UserDisconnect {
    pub user_id: usize,
}

#[derive(Debug, Deserialize)]
pub struct JoinRoomRequest {
    pub room_id: String,
    pub user_name: String,
    pub user_id: usize,
}

#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserDto>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserListRequest {}
