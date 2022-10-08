use std::str::FromStr;

use futures::channel::{mpsc, oneshot};
use futures::{select, SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::{Message, WebSocketError};
use wasm_bindgen_futures::spawn_local;

use pixessions::{Action, Session};

use crate::{Error, SESSION_ENDPOINT, SESSION_WS};

// Removed i, I, o, O -> 48 chars
// !! MUST be sorted by ascii values
static CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Code {
    pub session_id: u64,
    pub quiz_id: u64,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = 0u128;

        for char in s.chars() {
            let index = CHARS.iter().position(|c| char::from(*c) == char).ok_or(())?;
            code = code * (CHARS.len() as u128) + (index as u128);
        }

        // It is important the quiz id is the last part of the code in binary (most significant bits)
        // As quiz id's are usually small, this reduces the length of the ascii code.
        Ok(Code { session_id: (code & 0xFFFFFFFFFFFFFFFF) as u64, quiz_id: (code >> 64) as u64 })
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let mut code = self.session_id as u128 + ((self.quiz_id as u128) << 64);

        while code != 0 {
            let rem = code % CHARS.len() as u128;
            string.insert(0, char::from(CHARS[rem as usize]));
            code /= CHARS.len() as u128;
        }
        string
    }
}

pub async fn create_session(quiz_id: u64) -> Result<u64, Error> {
    let endpoint = format!("{SESSION_ENDPOINT}/{quiz_id}");
    let text = reqwest::Client::new().post(&endpoint).send().await?.text().await?;

    u64::from_str(&text).map_err(|_| Error::InvalidSession)
}

pub async fn query_sessions() -> Result<Vec<Session>, Error> {
    let endpoint = format!("{SESSION_ENDPOINT}");
    Ok(reqwest::Client::new().get(&endpoint).send().await?.json().await?)
}

pub struct WebsocketTask {
    _marker: oneshot::Sender<()>,
    sender: mpsc::Sender<Action>,
}

impl WebsocketTask {
    pub fn send(&self, action: &Action) {
        let (mut sender, action) = (self.sender.clone(), action.clone());
        spawn_local(async move { sender.send(action).await.unwrap() });
    }

    pub fn handle(message: Option<Result<Message, WebSocketError>>) -> Result<Session, Error> {
        match message.ok_or(Error::WsClosed)?.map_err(|_| Error::WsFailure)? {
            Message::Bytes(_) => Err(Error::WsBytes),
            Message::Text(text) => Ok(serde_json::from_str(&text)?),
        }
    }

    pub async fn run(
        mut actions: mpsc::Receiver<Action>,
        mut cancel: oneshot::Receiver<()>,
        callback: impl Fn(Result<Session, Error>),
        ws: WebSocket,
    ) {
        let (mut sender, receiver) = ws.split();
        let mut receiver = receiver.fuse();

        loop {
            select! {
                message = receiver.next() => {
                    callback(Self::handle(message));
                },
                action = actions.next() => {
                    let message = Message::Text(serde_json::to_string(&action).unwrap());
                    sender.send(message).await.unwrap();
                },
                _ = cancel => break
            }
        }

        let ws = sender.reunite(receiver.into_inner()).unwrap();
        ws.close(Some(1000), Some("session closed by user")).unwrap();
    }

    pub fn new(
        session_id: u64,
        callback: impl Fn(Result<Session, Error>) + 'static,
    ) -> Result<Self, Error> {
        let endpoint = format!("{SESSION_WS}/{session_id}");
        let ws = WebSocket::open(&endpoint).map_err(|_| Error::WsConnection)?;

        let (_marker, cancel) = oneshot::channel();
        let (sender, actions) = mpsc::channel(100);

        spawn_local(async move { Self::run(actions, cancel, callback, ws).await });

        Ok(Self { _marker, sender })
    }
}
