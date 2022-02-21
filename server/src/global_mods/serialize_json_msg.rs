use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct InvalidMessage {
    pub code: usize,
    pub error: &'static str,
    pub message: String,
}
