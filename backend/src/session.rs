use std::collections::HashMap;

use tokio::sync::mpsc;
use warp::ws::Message;

use api::{Player, Stage};

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub struct SessionData {
    pub host: Sender,
    pub manager: Option<Sender>,

    pub quiz_id: i64,
    pub rounds: usize,
    pub stage: Stage,

    pub players: HashMap<u64, Player>,
    pub current_id: u64,
}
