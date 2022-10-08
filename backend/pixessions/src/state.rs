use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use tokio::sync::Mutex;

use pixessions::Session;

pub struct State {
    pub session: Session,
    pub connections: HashMap<u32, SplitSink<WebSocket, Message>>,
}

impl State {
    pub fn new(session: Session) -> Self {
        Self { session, connections: HashMap::new() }
    }
}

pub type Local = Arc<Mutex<State>>;
pub type Global = Arc<Mutex<HashMap<u32, Local>>>;
