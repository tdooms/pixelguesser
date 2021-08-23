use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;
use tokio::sync::Mutex;
use warp::ws::Message;

use shared::{Stage, Player};

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Clone, Debug)]
pub struct InternalSession {
    pub quiz_id: u64,
    pub host: Option<Sender>,
    pub manager: Option<Sender>,

    pub players: Vec<Player>,
    pub stage: Stage,
}

pub type State = Arc<Mutex<HashMap<u64, InternalSession>>>;
