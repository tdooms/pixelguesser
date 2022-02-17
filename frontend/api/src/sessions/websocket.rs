use std::fmt::Debug;

use cynic::serde_json;
use futures::channel::{mpsc, oneshot};
use futures::{select, SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use sessions::{Request, Response};

use crate::Error;

pub struct WebsocketTask {
    responder: mpsc::UnboundedSender<Result<Message, WebSocketError>>,
    cancel: oneshot::Sender<()>,
}

impl WebsocketTask {
    pub fn send(&mut self, request: &Request) {
        log::debug!("ws request: {:?}", request);

        let str = serde_json::to_string(request).unwrap();
        let mut cloned = self.responder.clone();

        spawn_local(async move { cloned.send(Ok(Message::Text(str))).await.unwrap() });
    }

    fn handle(
        result: Result<Message, WebSocketError>,
        callback: &Callback<Result<Response, Error>>,
        onerror: &Callback<Error>,
    ) {
        match result {
            Ok(Message::Text(m)) => match serde_json::from_str(&m) {
                Ok(response) => {
                    log::debug!("ws response: {:?}", response);
                    callback.emit(Ok(response))
                }
                Err(err) => callback.emit(Err(Error::Serde(err))),
            },
            Ok(Message::Bytes(_)) => {
                log::warn!("deserializing bytes over ws not supported")
            }
            Err(Web) => {
                onerror.emit(Error::WsError);
            }
        }
    }

    pub fn create(
        url: impl AsRef<str>,
        callback: Callback<Result<Response, Error>>,
        onerror: Callback<Error>,
    ) -> Self {
        log::debug!("connecting to {}", url.as_ref());
        let ws = WebSocket::open(url.as_ref()).unwrap();

        let (sink, stream) = ws.split();
        let (responder, receiver) = mpsc::unbounded();

        let (cancel_send, mut cancel_recv) = oneshot::channel();

        let mut stream = stream.fuse();

        spawn_local(async move {
            receiver.forward(sink).await;
            log::debug!("should be dropped (5)")
        });

        spawn_local(async move {
            while let Some(m) = select! {
                message = stream.next() => message,
                _ = cancel_recv => return
            } {
                Self::handle(m, &callback, &onerror)
            }
            log::debug!("should be dropped (6)");
        });

        Self { responder, cancel: cancel_send }
    }
}
