use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use pixessions::Session;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum Mode {
    Couch,
    Online,
    Solo,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Couch
    }
}

#[derive(Default)]
pub struct State {
    pub session: Session,
    pub mode: Mode,
    pub quiz: u32,
    pub connections: HashMap<u32, SplitSink<WebSocket, Message>>,
}

impl State {
    pub fn new(mode: Mode, quiz: u32) -> Self {
        State { mode, quiz, ..Default::default() }
    }
}

pub type Local = Arc<Mutex<State>>;
pub type Global = Arc<Mutex<HashMap<u32, Local>>>;
