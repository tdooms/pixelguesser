use crate::Session;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

pub type Responder = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Default, Clone)]
pub struct Global {
    pub hosts: Arc<Mutex<HashMap<u64, (Responder, Arc<Mutex<Session>>)>>>,
}
