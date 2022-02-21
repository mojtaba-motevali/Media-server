use crate::user::User;
use mediasoup::transport::Transport;
use mediasoup::webrtc_transport::WebRtcTransport;

impl User {
    ///
    /// This function inserts new transport for user.
    ///
    pub fn insert_transport(&mut self, transport: WebRtcTransport, is_consumer: bool) {
        if is_consumer {
            self.current_ct_id = transport.id().to_string();
            println!("inserting consumer transport id; {:?}", transport.id());
            self.consumer_transports
                .insert(self.current_ct_id.clone(), transport);
        } else {
            self.current_pt_id = transport.id().to_string();
            self.producer_transports
                .insert(self.current_pt_id.clone(), transport);
        }
    }
    ///
    /// this function returns user's transports.
    ///
    pub fn get_transports(&self, is_consumer: bool) -> Vec<WebRtcTransport> {
        let mut vec = vec![];
        if is_consumer {
            for transport in self.consumer_transports.values() {
                vec.push(transport.clone());
            }
        } else {
            for transport in self.producer_transports.values() {
                vec.push(transport.clone());
            }
        }
        vec
    }
}
