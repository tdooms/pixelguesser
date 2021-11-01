mod handle;
mod internal;

use std::net::SocketAddr;

use clap::{AppSettings, Clap};
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{WebSocket, Ws};
use warp::Filter;

use crate::handle::handle_request;
use crate::internal::State;

/// sessions is a server to manage pixelguesser game sessions
#[derive(Clap)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets the port to be used
    #[clap(short, long, default_value = "8000")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
}

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

    // TODO: use id to identify the disconnect and handle it

    log::debug!("client disconnected");
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    let state = State::default();
    let state = warp::any().map(move || state.clone());

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(state)
        .map(|ws: Ws, state: State| ws.on_upgrade(|socket| start_socket(socket, state)));

    let address: SocketAddr = format!("{}:{}", opts.address, opts.port).parse().unwrap();
    warp::serve(ws).run(address).await;
}
