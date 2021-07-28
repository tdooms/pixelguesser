use std::net::SocketAddrV4;

use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{WebSocket, Ws};
use warp::Filter;

use crate::handle::handle_request;
use crate::state::State;

mod db;
mod error;
mod handle;
mod session;
mod state;

async fn start_socket(socket: WebSocket, state: State) {
    log::debug!("new client connected");

    let (sender, mut receiver) = socket.split();
    let (proxy_sender, proxy_receiver) = mpsc::unbounded_channel();
    let proxy_receiver = UnboundedReceiverStream::new(proxy_receiver);

    tokio::task::spawn(proxy_receiver.forward(sender));

    while let Some(message) = receiver.next().await {
        match message {
            Ok(message) if message.is_text() => {
                handle_request(message.as_bytes(), &state, &proxy_sender).await
            }
            Ok(message) if message.is_ping() => {
                log::debug!("pong back not implemented")
            }
            Ok(_) => log::warn!("unsupported websocket message type"),
            Err(error) => log::error!("websocket error: {}", error),
        }
    }

    log::debug!("client disconnected");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let uri = std::env::var("DATABASE_URL")?;

    let address = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8001".to_owned());
    let address = address.parse::<SocketAddrV4>()?;

    let state = State::new(&uri).await?;
    let state = warp::any().map(move || state.clone());

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(state)
        .map(|ws: Ws, state: State| ws.on_upgrade(|socket| start_socket(socket, state)));

    warp::serve(ws).run(address).await;
    Ok(())
}
