use futures::channel::mpsc;

use futures::{SinkExt, StreamExt};
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use sessions::{Request, Response};

pub struct WebsocketTask {
    responder: mpsc::UnboundedSender<Result<Message, WebSocketError>>,
}

impl WebsocketTask {
    pub fn send(&mut self, request: &Request) {
        log::debug!("ws request: {:?}", request);
        let str = serde_json::to_string(request).unwrap();

        let mut cloned = self.responder.clone();

        spawn_local(async move { cloned.send(Ok(Message::Text(str))).await.unwrap() });
    }

    pub fn create(url: impl AsRef<str>, callback: Callback<Response>) -> Self {
        log::debug!("connecting to {}", url.as_ref());
        let ws = WebSocket::open(url.as_ref()).unwrap();

        let (sink, mut stream) = ws.split();
        let (responder, receiver) = mpsc::unbounded();

        spawn_local(async move {
            receiver.forward(sink).await.unwrap();
        });

        spawn_local(async move {
            while let Some(m) = stream.next().await {
                match m {
                    Ok(Message::Text(m)) => match serde_json::from_str(&m) {
                        Ok(response) => {
                            log::debug!("ws response: {:?}", response);
                            callback.emit(response)
                        }
                        Err(_err) => log::error!("TODO: handle deserialize error"),
                    },
                    Ok(Message::Bytes(_m)) => log::error!("TODO: handle deserialize bytes over ws"),
                    Err(_e) => log::error!("TODO: handle ws error"),
                }
            }
        });

        Self { responder }
    }
}
