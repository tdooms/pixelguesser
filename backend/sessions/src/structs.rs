use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use warp::ws::Message;

use shared::Session;

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Clone, Debug)]
pub struct SessionData {
    pub host: Option<Sender>,
    pub manager: Option<Sender>,

    pub session: Session,
}

pub type State = Arc<Mutex<HashMap<u64, SessionData>>>;
