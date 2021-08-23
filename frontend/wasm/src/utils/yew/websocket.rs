use futures::channel::mpsc::UnboundedSender;
use reqwasm::websocket::{Message, WebSocket};
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use futures::StreamExt;
use shared::{Request, Response};

pub struct WebsocketTask {
    inner: UnboundedSender<Message>,
}

impl WebsocketTask {
    pub fn send(&self, request: Request) {
        log::error!("did not send request, TODO")
    }

    pub fn create(url: impl AsRef<str>, callback: Callback<Response>) -> Self {
        let ws = WebSocket::open(url.as_ref()).unwrap();

        let (sender, mut receiver) = (ws.sender, ws.receiver);

        spawn_local(async move {
            while let Some(m) = receiver.next().await {
                match m {
                    Ok(Message::Text(m)) => match serde_json::from_str(&m) {
                        Ok(response) => callback.emit(response),
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
