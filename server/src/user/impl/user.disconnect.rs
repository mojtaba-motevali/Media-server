use crate::user::User;
use mediasoup::transport::TransportGeneric;

impl User {
    ///
    /// this function is used to clean up the resources used by this user.
    /// TODO: move this to Drop trait
    ///
    pub async fn clean_up(&mut self) {
        for p in self.producers.values() {
            if !p.closed() {
                let _ = p.dump().await;
            }
        }
        self.producer_transports.clear();
        for c in self.consumers.values() {
            if !c.closed() {
                let _ = c.dump().await;
            }
        }
        self.consumers.clear();
        for transport in self.consumer_transports.values() {
            let _ = transport.dump().await;
        }
        self.consumer_transports.clear();
        for transport in self.producer_transports.values() {
            let _ = transport.dump().await;
        }
        self.producer_transports.clear();
        println!("User's resource cleaned");
    }
}
