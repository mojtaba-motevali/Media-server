use crate::global_mods::construct_message::build_error_message;
use crate::session::internal_msg::InternalErrorMessage;
use crate::session::WsSession;
use actix::*;

impl Handler<InternalErrorMessage> for WsSession {
    type Result = ();
    fn handle(&mut self, msg: InternalErrorMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            InternalErrorMessage::UnAuthorized(message) => {
                ctx.text(build_error_message((401, "UN_AUTHORIZED"), message));
            }
            // InternalErrorMessage::BadRequest(message) =>{
            //     ctx.text(build_error_message( (400,"BAD_REQUEST"),message));
            // },
            InternalErrorMessage::ServerFault(message) => {
                ctx.text(build_error_message((500, "SERVER_FAULT"), message));
            }
            InternalErrorMessage::InvalidJson(message) => {
                ctx.text(build_error_message((422, "INVALID_JSON_MESSAGE"), message));
            }
            InternalErrorMessage::NotFound(message) => {
                ctx.text(build_error_message((404, "Entity_NOT_FOUND"), message));
            }
        }
    }
}
