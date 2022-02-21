use mediasoup::router::RouterId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MediaServiceType {
    VOICE,
    CAMERA,
    SCREENSHARE,
}
#[derive(Serialize, Deserialize, Debug, Copy)]
pub struct MediaAppData {
    pub service_type: MediaServiceType,
    pub user_id: usize,
    pub router_id: RouterId,
}
impl Clone for MediaAppData {
    fn clone(&self) -> Self {
        Self {
            service_type: self.service_type.clone(),
            user_id: self.user_id,
            router_id: self.router_id.clone(),
        }
    }
}
