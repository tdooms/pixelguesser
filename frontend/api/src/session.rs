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

#[allow(dead_code)]
pub struct WebsocketTask {
    responder: mpsc::UnboundedSender<Result<Message, WebSocketError>>,
    cancel: oneshot::Sender<()>,
}

impl WebsocketTask {
    pub fn send(&mut self, action: &Action) {
        log::debug!("ws request: {:?}", action);

        let request = serde_json::to_string(action).unwrap();
        let mut responder = self.responder.clone();

        spawn_local(async move { responder.send(Ok(Message::Text(request))).await.unwrap() });
    }

    fn handle(result: Result<Message, WebSocketError>, callback: &Callback<Response>) {
        match result {
            Ok(Message::Text(m)) => {
                log::debug!("ws response: {:?}", m);
                match serde_json::from_str::<Response>(&m) {
                    Ok(response) => callback.emit(response),
                    Err(err) => log::warn!("ws error: {:?}", err),
                }
            }
            Ok(Message::Bytes(_)) => {
                log::warn!("deserializing bytes over ws not supported")
            }
            Err(_web) => {
                log::warn!("websocket error")
            }
        }
    }

    pub fn create(id: u32, callback: Callback<Response>) -> Self {
        let endpoint = format!("{SESSION_WS_ENDPOINT}/{id}");
        let ws = WebSocket::open(&endpoint).unwrap();
        log::debug!("connecting to {endpoint}");

        let (sink, stream) = ws.split();
        let (responder, receiver) = mpsc::unbounded();

        let (cancel_send, mut cancel_recv) = oneshot::channel();

        let mut stream = stream.fuse();

        spawn_local(async move {
            receiver.forward(sink).await.unwrap();
            log::debug!("should be dropped (5)")
        });

        spawn_local(async move {
            while let Some(m) = select! {
                message = stream.next() => message,
                _ = cancel_recv => return
            } {
                Self::handle(m, &callback)
            }
            log::debug!("should be dropped (6)");
        });

        Self { responder, cancel: cancel_send }
    }
}
