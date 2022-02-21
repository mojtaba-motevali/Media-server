pub mod args;

#[path = "internal.dto.rs"]
pub mod internal;

#[path = "request/connect_consumer_transport.dto.rs"]
pub mod c_c_transport_request;

#[path = "request/connect_producer_transport.dto.rs"]
pub mod c_p_transport_request;

#[path = "request/consumer_close.dto.rs"]
pub mod consumer_close_request;

#[path = "request/consumer_pause.dto.rs"]
pub mod consumer_pause_request;

#[path = "request/consumer_resume.dto.rs"]
pub mod consumer_resume_request;

#[path = "request/rtp_capability_settings.dto.rs"]
pub mod rtp_capability_settings_request;

#[path = "request/producer_close.dto.rs"]
pub mod producer_close_request;

#[path = "request/producer_pause.dto.rs"]
pub mod producer_pause_request;

#[path = "request/producer_resume.dto.rs"]
pub mod producer_resume_request;

#[path = "request/produce.dto.rs"]
pub mod produce_request;

#[path = "request/broadcast_message.dto.rs"]
pub mod broadcast_message_request;

#[path = "request/private_message.dto.rs"]
pub mod private_message_request;

#[path = "request/new_user.dto.rs"]
pub mod new_user_request;

/// response dtos

#[path = "response/private_message.dto.rs"]
pub mod private_message_response;

#[path = "response/broadcast_message.dto.rs"]
pub mod broadcast_message_response;

#[path = "response/active_speaker.dto.rs"]
pub mod active_speaker_response;

#[path = "response/close_inactive_room.dto.rs"]
pub mod close_inactive_room_response;

#[path = "response/connected_consumer_transport.dto.rs"]
pub mod c_c_transport_response;

#[path = "response/connected_producer_transport.dto.rs"]
pub mod c_p_transport_response;

#[path = "response/consumer_pause.dto.rs"]
pub mod consumer_pause_response;

#[path = "response/consumer_resume.dto.rs"]
pub mod consumer_resume_response;

#[path = "response/consumed.dto.rs"]
pub mod consumed_response;

#[path = "response/consumer_close.dto.rs"]
pub mod consumer_close_response;

#[path = "response/join_room.dto.rs"]
pub mod join_room_response;

#[path = "response/new_user.dto.rs"]
pub mod new_user_response;

#[path = "response/new_webrtc_transport.dto.rs"]
pub mod new_webrtc_transport_response;

#[path = "response/produced.dto.rs"]
pub mod produced_response;

#[path = "response/user_list.dto.rs"]
pub mod user_list_response;
