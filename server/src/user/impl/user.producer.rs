use crate::media::app_data::media_service::MediaAppData;
use crate::room::dto::args::produce_dto::ProduceArgsDto;
use crate::room::dto::c_p_transport_request::ConnectProducerTransportRequest;
use crate::user::User;
use mediasoup::data_structures::AppData;
use mediasoup::producer::{Producer, ProducerId, ProducerOptions};
use mediasoup::transport::ProduceError;
use mediasoup::transport::Transport;
use mediasoup::webrtc_transport::WebRtcTransport;
use mediasoup::webrtc_transport::WebRtcTransportRemoteParameters;
use mediasoup::worker::RequestError;

impl User {
    ///
    /// This function inserts the provided producer into producer list.
    ///
    pub fn insert_producer(&mut self, producer: Producer) {
        println!("inserting producer  {:?} ", producer.id());
        self.producers.insert(producer.id(), producer);
    }
    ///
    /// This function removes the provided producer id from list of producers.
    ///
    pub fn remove_producer(&mut self, producer_id: ProducerId) -> Option<Producer> {
        self.producers.remove(&producer_id)
    }
    ///
    /// This function connects backend's producer transport to client side transport
    ///
    pub async fn connect_producer_transport(
        &self,
        connect_msg: ConnectProducerTransportRequest,
    ) -> Result<(), RequestError> {
        let ConnectProducerTransportRequest {
            user_id: _,
            dtls_parameters,
            transport_id,
        } = connect_msg;
        if let Some(transport) = self.producer_transports.get(&transport_id) {
            transport
                .connect(WebRtcTransportRemoteParameters { dtls_parameters })
                .await?;
        }
        Ok(())
    }
    pub async fn create_producer(
        &mut self,
        producer_args_options: ProduceArgsDto,
    ) -> Result<Producer, ProduceError> {
        let ProduceArgsDto {
            rtp_parameters,
            user_id,
            service_type,
            kind,
        } = producer_args_options;
        let transport: &WebRtcTransport =
            self.producer_transports.get(&self.current_pt_id).unwrap();
        let mut produce_options = ProducerOptions::new(kind, rtp_parameters);
        let app_data = MediaAppData {
            service_type: service_type,
            user_id: user_id,
            router_id: transport.router().id(),
        };
        produce_options.app_data = AppData::new(app_data.clone());
        let producer: Producer = transport.produce(produce_options).await?;
        self.producers.insert(producer.id(), producer.clone());
        return Ok(producer);
    }
    pub fn get_producers(&self) -> Vec<Producer> {
        let mut producers: Vec<Producer> = vec![];
        self.producers
            .values()
            .for_each(|prod| producers.push(prod.clone()));
        return producers;
    }

    pub async fn pause_producer(&self, producer_id: ProducerId) -> Result<(), RequestError> {
        if let Some(producer) = self.producers.get(&producer_id) {
            if !producer.paused() {
                producer.pause().await?;
            }
        }
        Ok(())
    }

    pub async fn resume_producer(&self, producer_id: ProducerId) -> Result<(), RequestError> {
        if let Some(producer) = self.producers.get(&producer_id) {
            if producer.paused() {
                producer.resume().await?;
            }
        }
        Ok(())
    }
}