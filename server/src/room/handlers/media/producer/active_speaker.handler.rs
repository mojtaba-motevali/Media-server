use crate::media::media_struct::ActiveSpeaker;
use crate::room::message::RoomActiveSpeakerDetector;
use crate::room::Room;
use actix::prelude::*;
///
/// This handler is used to determine room's active speaker.
///
impl Handler<RoomActiveSpeakerDetector> for Room {
    type Result = ();
    fn handle(&mut self, msg: RoomActiveSpeakerDetector, _ctx: &mut Context<Self>) -> Self::Result {
        if self.active_speaker_elected.as_ref().is_none()
            || (self.active_speaker_elected.as_ref().unwrap().volume > msg.volume)
        {
            let active_speaker = ActiveSpeaker {
                producer_id: msg.producer_id,
                volume: msg.volume,
                user_id: msg.user_id,
            };
            self.active_speaker_elected.replace(active_speaker);
        }
    }
}
