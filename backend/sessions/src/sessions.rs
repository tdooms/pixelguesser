use crate::shared::State;
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum Mode {
    Couch,
    // Online,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Couch
    }
}

#[derive(Default)]
pub struct Session {
    pub state: State,
    pub mode: Mode,
    pub quiz: u64,
    pub connections: HashMap<u64, SplitSink<WebSocket, Message>>,
}

impl Session {
    pub fn new(mode: Mode, quiz: u64) -> Self {
        Session { mode, quiz, ..Default::default() }
    }
}

#[derive(Default, Clone)]
pub struct Global {
    pub sessions: Arc<Mutex<HashMap<u64, Arc<Mutex<Session>>>>>,
}
