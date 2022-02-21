use std::sync::{Arc, Mutex};

pub struct AppState {
    rooms: Arc<Mutex<Vec<String>>>,
}
