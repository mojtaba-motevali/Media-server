use crate::media::app_data::media_service::MediaAppData;
use crate::room::dto::args::consume_dto::ConsumeArgsDto;
use crate::room::dto::c_c_transport_request::ConnectConsumerTransportRequest;
use crate::room::dto::consumer_pause_response::ConsumerPauseResponse;
use crate::room::dto::consumer_resume_response::ConsumerResumeResponse;
use crate::user::User;
use mediasoup::consumer::{Consumer, ConsumerId, ConsumerOptions};
use mediasoup::data_structures::AppData;
use mediasoup::transport::Transport;
use mediasoup::webrtc_transport::WebRtcTransportRemoteParameters;
use mediasoup::worker::RequestError;
impl User {
    ///
    /// This function inserts the provided consumer into consumer list.
    ///
    pub fn insert_consumer(&mut self, consumer: Consumer) {
        self.consumers.insert(consumer.id(), consumer);
    }
    ///
    /// This function removes the provided consumer id from list of consumers.
    ///
    pub fn remove_consumer(&mut self, consumer_id: ConsumerId) -> Option<Consumer> {
        if let Some(consumer) = self.consumers.remove(&consumer_id) {
            return Some(consumer);
        } else {
            println!("CONSUMER NOT FOUND");
            None
        }
    }

    ///
    /// This function connects backend's consumer transport to client side transport.
    ///
    pub async fn connect_consumer_transport(
        &self,
        connect_msg: ConnectConsumerTransportRequest,
    ) -> Result<(), RequestError> {
        let ConnectConsumerTransportRequest {
            user_id: _,
            dtls_parameters,
            transport_id,
        } = connect_msg;
        if let Some(transport) = self.consumer_transports.get(&transport_id) {
            transport
                .connect(WebRtcTransportRemoteParameters { dtls_parameters })
                .await?;
        }
        Ok(())
    }
    ///
    /// This function used to create consumer using provided options
    ///
    pub async fn create_consumer(&self, options: ConsumeArgsDto) -> Result<Consumer, String> {
        if let Some(transport) = self.consumer_transports.get(&self.current_ct_id) {
            println!("creating consumer from transport id; {:?}", transport.id());
            let mut consume_options = ConsumerOptions::new(
                options.producer_id,
                self.self_rtp_capabilities.clone().unwrap(),
            );
            consume_options.app_data = AppData::new(MediaAppData {
                user_id: options.user_id,
                service_type: options.service_type,
                router_id: transport.router().id(),
            });
            consume_options.paused = true;
            let consumer = match transport.consume(consume_options).await {
                Ok(consumer) => consumer,
                Err(error) => return Err(error.to_string()),
            };
            let id = consumer.id();
            let user_id = self.id;
            let ws_actor = self.ws_actor_addr.clone();
            consumer
                .on_producer_resume(move || {
                    ws_actor.do_send(ConsumerResumeResponse { user_id, id });
                })
                .detach();
            let ws_actor = self.ws_actor_addr.clone();
            consumer
                .on_producer_pause(move || ws_actor.do_send(ConsumerPauseResponse { user_id, id }))
                .detach();
            return Ok(consumer);
        } else {
            Err("transport_not_found".to_string())
        }
    }

    ///
    /// This function used to pause the stream of provided consumer id
    ///
    pub async fn pause_consumer(&self, consumer_id: ConsumerId) -> Result<(), RequestError> {
        if let Some(consumer) = self.consumers.get(&consumer_id) {
            if !consumer.paused() {
                consumer.pause().await?;
            }
        } else {
            println!("WARN%: consumer not found");
        }
        Ok(())
    }

    ///
    /// This function used to resume the paused stream of provided consumer id
    ///
    pub async fn resume_consumer(&self, consumer_id: ConsumerId) -> Result<(), RequestError> {
        if let Some(consumer) = self.consumers.get(&consumer_id) {
            if consumer.paused() {
                consumer.resume().await?;
            }
        } else {
            println!("WARN%: consumer not found");
        }
        Ok(())
    }
}
