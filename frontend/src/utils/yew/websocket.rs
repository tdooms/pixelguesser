use std::fmt::Debug;
use std::marker::PhantomData;

use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::{select, SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;
use yew_agent::Dispatched;

use crate::{Error, ErrorAgent};

pub struct WebsocketTask<Req: Serialize + Debug, Res: 'static + DeserializeOwned + Debug> {
    responder: mpsc::UnboundedSender<Result<Message, WebSocketError>>,
    cancel: oneshot::Sender<()>,
    req: PhantomData<Req>,
    res: PhantomData<Res>,
}

impl<Req: Serialize + Debug, Res: 'static + DeserializeOwned + Debug> WebsocketTask<Req, Res> {
    pub fn send(&mut self, request: &Req) {
        log::debug!("ws request: {:?}", request);

        let str = serde_json::to_string(request).unwrap();
        let mut cloned = self.responder.clone();

        spawn_local(async move { cloned.send(Ok(Message::Text(str))).await.unwrap() });
    }

    pub fn create(url: impl AsRef<str>, callback: Callback<Res>) -> Self {
        let mut errors = ErrorAgent::dispatcher();

        log::debug!("connecting to {}", url.as_ref());
        let ws = WebSocket::open(url.as_ref()).unwrap();

        let (sink, mut stream) = ws.split();
        let (responder, receiver) = mpsc::unbounded();

        let (cancel_send, mut cancel_recv) = oneshot::channel();

        spawn_local(async move {
            select! {
                _ = receiver.forward(sink) => {log::debug!("receiver done")},
                _ = cancel_recv => {log::debug!("sender dropped")}
            }
        });

        spawn_local(async move {
            while let Some(m) = stream.next().await {
                match m {
                    Ok(Message::Text(m)) => match serde_json::from_str(&m) {
                        Ok(response) => {
                            log::debug!("ws response: {:?}", response);
                            callback.emit(response)
                        }
                        Err(err) => log::error!("deserialize error: {:?}", err),
                    },
                    Ok(Message::Bytes(_)) => {
                        log::warn!("deserializing bytes over ws not supported")
                    }
                    Err(WebSocketError::ConnectionError) => {
                        errors.send(Error::WebSocket("connection error".to_owned()))
                    }
                    Err(WebSocketError::ConnectionClose(_)) => errors
                        .send(Error::WebSocket("connection closed / cannot connect".to_owned())),
                    Err(WebSocketError::MessageSendError(_)) => {
                        errors.send(Error::WebSocket("send error".to_owned()))
                    }
                    Err(_) => unreachable!(),
                }
            }
        });

        Self {
            responder,
            req: PhantomData::default(),
            res: PhantomData::default(),
            cancel: cancel_send,
        }
    }
}
