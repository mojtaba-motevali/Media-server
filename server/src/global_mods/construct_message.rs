use serde_json;

use super::serialize_json_msg::InvalidMessage;

pub fn build_error_message(tuple: (usize, &'static str), message: String) -> String {
    let msg = InvalidMessage {
        code: tuple.0,
        error: tuple.1,
        message,
    };
    serde_json::to_string(&msg).unwrap()
}
