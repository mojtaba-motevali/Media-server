#[path = "synchronization.handler.rs"]
pub mod synchronization;

#[path = "rtp_capability_settings.handler.rs"]
pub mod rtp_capability_settings;

#[path = "producer/new_producer.handler.rs"]
pub mod new_producer;

#[path = "producer/close_producer.handler.rs"]
pub mod close_producer;

#[path = "producer/active_speaker.handler.rs"]
pub mod active_speaker;

#[path = "producer/connect_producer.handler.rs"]
pub mod connect_producer;

#[path = "producer/produce.handler.rs"]
pub mod produce;

#[path = "producer/producer_pause.handler.rs"]
pub mod producer_pause;

#[path = "producer/producer_resume.handler.rs"]
pub mod producer_resume;

#[path = "consumer/consumer_connect.handler.rs"]
pub mod consumer_connect;

#[path = "consumer/consumer_pause.handler.rs"]
pub mod consumer_pause;

#[path = "consumer/consumer_resume.handler.rs"]
pub mod consumer_resume;

#[path = "consumer/consumer_close.handler.rs"]
pub mod consumer_close;
