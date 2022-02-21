use crate::media::media_struct::ActiveSpeaker;
use crate::room::dto::c_c_transport_response::ConnectedConsumerTransportResponse;
use crate::room::dto::c_p_transport_response::ConnectedProducerTransportResponse;
use crate::room::dto::consumed_response::ConsumedResponse;
use crate::room::dto::consumer_close_response::ConsumerCloseResponse;
use crate::room::dto::consumer_pause_response::ConsumerPauseResponse;
use crate::room::dto::consumer_resume_response::ConsumerResumeResponse;
use crate::room::dto::new_webrtc_transport_response::NewWebrtcTransportResponse;
use crate::room::dto::produced_response::{AudioProducedResponse, VideoProducedResponse};
use crate::session::socket_json_msg::ServerJsonMessage;
use crate::session::WsSession;
use actix::*;

impl Handler<ActiveSpeaker> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ActiveSpeaker, ctx: &mut Self::Context) -> Self::Result {
        let ActiveSpeaker {
            producer_id,
            volume,
            user_id,
        } = msg;
        ctx.notify(ServerJsonMessage::ActiveSpeaker(ActiveSpeaker {
            producer_id,
            volume,
            user_id,
        }));
    }
}

impl Handler<ConnectedConsumerTransportResponse> for WsSession {
    type Result = ();
    fn handle(
        &mut self,
        msg: ConnectedConsumerTransportResponse,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        ctx.notify(ServerJsonMessage::ConnectedConsumerTransport(msg));
    }
}

impl Handler<ConnectedProducerTransportResponse> for WsSession {
    type Result = ();
    fn handle(
        &mut self,
        msg: ConnectedProducerTransportResponse,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        ctx.notify(ServerJsonMessage::ConnectedProducerTransport(msg));
    }
}

impl Handler<ConsumedResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ConsumedResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::Consumed(msg));
    }
}

impl Handler<ConsumerCloseResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ConsumerCloseResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::ConsumerClose(msg));
    }
}

impl Handler<ConsumerPauseResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ConsumerPauseResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::ConsumerPause(msg));
    }
}

impl Handler<ConsumerResumeResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: ConsumerResumeResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::ConsumerResume(msg));
    }
}

impl Handler<NewWebrtcTransportResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: NewWebrtcTransportResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::NewWebrtcTransport(msg));
    }
}

impl Handler<AudioProducedResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: AudioProducedResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::AudioProduced(msg));
    }
}

impl Handler<VideoProducedResponse> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: VideoProducedResponse, ctx: &mut Self::Context) -> Self::Result {
        ctx.notify(ServerJsonMessage::VideoProduced(msg));
    }
}
