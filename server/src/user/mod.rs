pub mod r#impl;

use actix::prelude::*;
use actix::Message;
use serde::{Deserialize, Serialize};

use crate::session::WsSession;
use mediasoup::consumer::{Consumer, ConsumerId};
use mediasoup::producer::{Producer, ProducerId};
use mediasoup::rtp_parameters::RtpCapabilities;
use mediasoup::webrtc_transport::WebRtcTransport;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Message, Debug, Clone)]
#[rtype(result = "()")]
pub struct UserDto {
    pub id: Option<usize>,
    pub name: Option<String>,
}
///
/// User structure.
/// This structure is used to maintain user's state.
/// this structure includes consumer transports, producer transports and current transport index used by system.actix
/// also includes List of producers and consumers to be used in order to manage streams.
///
pub struct User {
    pub id: usize,
    pub name: String,
    pub ws_actor_addr: Addr<WsSession>,
    pub consumer_transports: HashMap<String, WebRtcTransport>,
    pub producer_transports: HashMap<String, WebRtcTransport>,
    pub current_ct_id: String,
    pub current_pt_id: String,
    pub self_rtp_capabilities: Option<RtpCapabilities>,
    pub producers: HashMap<ProducerId, Producer>,
    pub consumers: HashMap<ConsumerId, Consumer>,
}
