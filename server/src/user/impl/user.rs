use crate::session::WsSession;
use crate::user::User;
use actix::prelude::*;
use std::collections::HashMap;

impl User {
    pub fn new(id: usize, name: String, ws_addr: Addr<WsSession>) -> User {
        User {
            id,
            name,
            ws_actor_addr: ws_addr,
            consumer_transports: HashMap::new(),
            producer_transports: HashMap::new(),
            current_ct_id: String::new(),
            current_pt_id: String::new(),
            producers: HashMap::new(),
            consumers: HashMap::new(),
            self_rtp_capabilities: None,
        }
    }
}
