use mediasoup::router::RouterId;

#[derive(Debug, Copy)]
pub struct TransportAppData {
    pub is_consumer: bool,
    pub router_id: RouterId,
}
impl Clone for TransportAppData {
    fn clone(&self) -> Self {
        Self {
            is_consumer: self.is_consumer,
            router_id: self.router_id.clone(),
        }
    }
}
