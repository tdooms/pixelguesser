use std::str::FromStr;

use futures::channel::{mpsc, oneshot};
use futures::{select, SinkExt, StreamExt};
use gloo::net::websocket::futures::WebSocket;
use gloo::net::websocket::{Message, WebSocketError};
use reqwest::Client;
use wasm_bindgen_futures::spawn_local;
use base64::engine::fast_portable::{FastPortable, FastPortableConfig};
use base64::alphabet::URL_SAFE;

use pixessions::{Action, Session};

use crate::{Error, SESSION_ENDPOINT, WEBSOCKET_ENDPOINT};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Code {
    pub session_id: u64,
    pub quiz_id: u64,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let engine = FastPortable::from(&URL_SAFE, FastPortableConfig::new());

        let decoded = base64::decode_engine(s, &engine).unwrap();
        let code = u64::from_le_bytes(decoded.try_into().unwrap());

        Ok(Code{session_id: code >> 32, quiz_id: code & 0xFFFFFFFF})
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let engine = FastPortable::from(&URL_SAFE, FastPortableConfig::new());
        let code = (self.quiz_id << 32 | self.session_id).to_le_bytes();

        base64::encode_engine(&code, &engine)
    }
}

pub async fn create_session(quiz_id: u64) -> Result<u64, Error> {
    let url = format!("{SESSION_ENDPOINT}/{quiz_id}/Couch");

    let text = Client::new()
        .post(&url)
        .send()
        .await
        .map_err(|_| Error::Unreachable("pixessions", url))?
        .text()
        .await
        .map_err(|e| Error::InvalidResponse("pixessions", e.to_string()))?;

    u64::from_str(&text).map_err(|e| Error::InvalidResponse("pixessions", e.to_string()))
}

pub async fn query_sessions() -> Result<Vec<Session>, Error> {
    let url = format!("{SESSION_ENDPOINT}");

    Client::new()
        .get(&url)
        .send()
        .await
        .map_err(|_| Error::Unreachable("pixessions", url))?
        .json()
        .await
        .map_err(|e| Error::InvalidResponse("pixessions", e.to_string()))
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
            Message::Text(text) => Ok(serde_json::from_str(&text)
                .map_err(|e| Error::InvalidResponse("pixessions", e.to_string()))?),
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
        let url = format!("{WEBSOCKET_ENDPOINT}/{session_id}");
        let ws = WebSocket::open(&url).map_err(|_| Error::Unreachable("ws", url))?;

        let (_marker, cancel) = oneshot::channel();
        let (sender, actions) = mpsc::channel(100);

        spawn_local(async move { Self::run(actions, cancel, callback, ws).await });

        Ok(Self { _marker, sender })
    }
}
