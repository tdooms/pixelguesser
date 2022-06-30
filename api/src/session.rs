use crate::{Error, SESSION_CREATE_ENDPOINT, SESSION_WS_ENDPOINT};
use futures::channel::{mpsc, oneshot};
use futures::{select, SinkExt, StreamExt};
use reqwasm::http::{Method, Request};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use sessions::{Action, Response};
use std::str::FromStr;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

// Removed i, I, o, O -> 48 chars
// !! MUST be sorted by ascii values
static CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZabcdefghjklmnpqrstuvwxyz";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Code {
    pub session_id: u32,
    pub quiz_id: u32,
}

impl FromStr for Code {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut code = 0u64;

        for char in s.chars() {
            let index = CHARS.iter().position(|c| char::from(*c) == char).ok_or(())?;
            code = code * (CHARS.len() as u64) + (index as u64);
        }

        // It is important the quiz id is the last part of the code in binary (most significant bits)
        // As quiz id's are usually small, this reduces the length of the ascii code.
        Ok(Code { session_id: (code & 0x00000000FFFFFFFF) as u32, quiz_id: (code >> 32) as u32 })
    }
}

impl ToString for Code {
    fn to_string(&self) -> String {
        let mut string = String::new();
        let mut code = self.session_id as u64 + ((self.quiz_id as u64) << 32);

        while code != 0 {
            let rem = code % CHARS.len() as u64;
            string.insert(0, char::from(CHARS[rem as usize]));
            code /= CHARS.len() as u64;
        }
        string
    }
}

pub async fn create_session(quiz_id: u32) -> Result<u32, Error> {
    let endpoint = format!("{SESSION_CREATE_ENDPOINT}/{quiz_id}");
    log::warn!("{endpoint}");
    let builder = Request::new(&endpoint).method(Method::GET);

    let response = builder.send().await?.text().await?;
    Ok(u32::from_str(&response)?)
}

pub struct WebsocketTask {
    _marker: oneshot::Sender<()>,
    sender: mpsc::Sender<Action>,
}

impl WebsocketTask {
    pub fn send(&mut self, action: &Action) {
        log::trace!("ws request: {:?}", action);

        let (mut sender, action) = (self.sender.clone(), action.clone());
        spawn_local(async move { sender.send(action).await.unwrap() });
    }

    pub fn handle(message: Option<Result<Message, WebSocketError>>) -> Result<Response, String> {
        match message.ok_or(String::from("none option"))?.map_err(|e| e.to_string())? {
            Message::Bytes(_) => Err("Websocket received bytes".to_string()),
            Message::Text(text) => serde_json::from_str(&text).map_err(|e| e.to_string()),
        }
    }

    pub async fn run(
        mut actions: mpsc::Receiver<Action>,
        mut cancel: oneshot::Receiver<()>,
        callback: Callback<Response>,
        ws: WebSocket,
    ) {
        let (mut sender, receiver) = ws.split();
        let mut receiver = receiver.fuse();

        loop {
            select! {
                message = receiver.next() => match Self::handle(message) {
                    Ok(response) => {
                        log::trace!("ws response: {:?}", response);
                        callback.emit(response);
                    },
                    Err(err) => log::error!("{}", err),
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

    pub fn new(session_id: u32, callback: Callback<Response>) -> Self {
        let endpoint = format!("{SESSION_WS_ENDPOINT}/{session_id}");
        let ws = WebSocket::open(&endpoint).unwrap();

        let (_marker, cancel) = oneshot::channel();
        let (sender, actions) = mpsc::channel(100);

        spawn_local(async move { Self::run(actions, cancel, callback, ws).await });

        Self { _marker, sender }
    }
}
