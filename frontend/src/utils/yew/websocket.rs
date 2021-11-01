use futures::channel::mpsc::UnboundedSender;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{Message, WebSocket};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use sessions::{Request, Response};

pub struct WebsocketTask {
    inner: UnboundedSender<Message>,
}

impl WebsocketTask {
    pub fn send(&mut self, request: &Request) {
        log::debug!("ws request: {:?}", request);
        let str = serde_json::to_string(request).unwrap();

        let mut cloned = self.inner.clone();

        spawn_local(async move { cloned.send(Message::Text(str)).await.unwrap() });
    }

    pub fn create(url: impl AsRef<str>, callback: Callback<Response>) -> Self {
        log::debug!("connecting to {}", url.as_ref());
        let ws = WebSocket::open(url.as_ref()).unwrap();

        let (sender, mut receiver) = (ws.sender, ws.receiver);

        spawn_local(async move {
            while let Some(m) = receiver.next().await {
                match m {
                    Ok(Message::Text(m)) => match serde_json::from_str(&m) {
                        Ok(response) => {
                            log::debug!("ws response: {:?}", response);
                            callback.emit(response)
                        }
                        Err(err) => log::error!("TODO: handle deserialize error"),
                    },
                    Ok(Message::Bytes(m)) => log::error!("TODO: handle deserialize bytes over ws"),
                    Err(e) => log::error!("TODO: handle ws error"),
                }
            }
        });

        Self { inner: sender }
    }
}
