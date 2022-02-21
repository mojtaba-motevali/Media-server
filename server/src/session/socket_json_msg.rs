use super::client_msg::{JoinRoomRequest, UserListRequest};
use actix::Message;
use serde::{Deserialize, Serialize};

use super::client_msg::UserDisconnect;
use crate::media::media_struct::ActiveSpeaker;
use crate::room::dto::broadcast_message_response::BroadcastMessageResponse;
use crate::room::dto::c_c_transport_response::ConnectedConsumerTransportResponse;
use crate::room::dto::c_p_transport_response::ConnectedProducerTransportResponse;
use crate::room::dto::consumed_response::ConsumedResponse;
use crate::room::dto::consumer_close_response::ConsumerCloseResponse;
use crate::room::dto::consumer_pause_response::ConsumerPauseResponse;
use crate::room::dto::consumer_resume_response::ConsumerResumeResponse;
use crate::room::dto::new_webrtc_transport_response::NewWebrtcTransportResponse;
use crate::room::dto::private_message_response::PrivateMessageResponse;
use crate::room::dto::produced_response::{AudioProducedResponse, VideoProducedResponse};
use crate::session::dto::join_room_response::JoinRoomResponse;

use crate::room::dto::broadcast_message_request::BroadcastMessageRequest;
use crate::room::dto::c_c_transport_request::ConnectConsumerTransportRequest;
use crate::room::dto::c_p_transport_request::ConnectProducerTransportRequest;
use crate::room::dto::consumer_pause_request::ConsumerPauseRequest;
use crate::room::dto::consumer_resume_request::ConsumerResumeRequest;
use crate::room::dto::new_user_response::NewUserResponse;
use crate::room::dto::private_message_request::PrivateMessageRequest;
use crate::room::dto::produce_request::ProduceRequest;
use crate::room::dto::producer_close_request::ProducerCloseRequest;
use crate::room::dto::producer_pause_request::ProducerPauseRequest;
use crate::room::dto::producer_resume_request::ProducerResumeRequest;
use crate::room::dto::rtp_capability_settings_request::RtpCapabilitiesSettings;
// use crate::room::dto::consumer_close_request::ConsumerCloseRequest;

#[derive(Serialize)]
#[serde(tag = "event")]
#[serde(rename_all = "snake_case")]
#[derive(Message)]
#[rtype(result = "()")]
pub enum ServerJsonMessage {
    UserDisconnect(UserDisconnect),
    JoinRoom(JoinRoomResponse),
    PrivateChat(PrivateMessageResponse),
    BroadcastMessage(BroadcastMessageResponse),
    ConnectedConsumerTransport(ConnectedConsumerTransportResponse),
    ConnectedProducerTransport(ConnectedProducerTransportResponse),
    Consumed(ConsumedResponse),
    VideoProduced(VideoProducedResponse),
    AudioProduced(AudioProducedResponse),
    ConsumerClose(ConsumerCloseResponse),
    ConsumerPause(ConsumerPauseResponse),
    ConsumerResume(ConsumerResumeResponse),
    NewUser(NewUserResponse),
    NewWebrtcTransport(NewWebrtcTransportResponse),
    ActiveSpeaker(ActiveSpeaker),
}

#[derive(Deserialize)]
#[serde(tag = "event")]
#[serde(rename_all = "snake_case")]
#[derive(Debug)]
// JsonEnum for Deserializing based on event
pub enum ClientJsonMessage {
    PrivateChat(PrivateMessageRequest),
    BroadcastMessage(BroadcastMessageRequest),
    JoinRoom(JoinRoomRequest),
    UserList(UserListRequest),
    SetRtpCapability(RtpCapabilitiesSettings),
    ConnectProducerTransport(ConnectProducerTransportRequest),
    Produce(ProduceRequest),
    ProducerResume(ProducerResumeRequest),
    ProducerPause(ProducerPauseRequest),
    ProducerClose(ProducerCloseRequest),
    ConnectConsumerTransport(ConnectConsumerTransportRequest),
    ConsumerResume(ConsumerResumeRequest),
    ConsumerPause(ConsumerPauseRequest),
}
