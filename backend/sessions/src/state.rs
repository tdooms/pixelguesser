use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{watch, RwLock};

use shared::Session;

pub struct SessionData {
    pub sender: watch::Sender<Session>,
    pub receiver: watch::Receiver<Session>
}

pub type Sessions = Arc<RwLock<HashMap<u64, SessionData>>>;