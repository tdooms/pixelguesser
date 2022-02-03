use std::net::SocketAddr;

use clap::Parser;
use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{WebSocket, Ws};
use warp::Filter;

mod state;

use crate::state::{Connection, Global};
use sessions::{Error, Session};

/// sessions is a server to manage pixelguesser game sessions
#[derive(Parser)]
#[clap(version = "1.0", author = "Thomas Dooms <thomas@dooms.eu>")]
struct Opts {
    /// Sets the port to be used
    #[clap(short, long, default_value = "8000")]
    port: u16,

    /// Sets the ip address to be used
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,
}

async fn start_socket(socket: WebSocket, global: Global) {
    log::info!("client connected");

    let (sink, mut stream) = socket.split();
    let (responder, receiver) = mpsc::unbounded_channel();
    let proxy = UnboundedReceiverStream::new(receiver);

    tokio::task::spawn(proxy.forward(sink));

    let mut connection = Connection { global, responder, local: None };

    while let Some(message) = stream.next().await {
        match message {
            Ok(message) if message.is_text() => match serde_json::from_slice(&message.as_bytes()) {
                Ok(request) => connection.request(request).await,
                Err(_) => connection.respond(&Err(Error::FaultyRequest)).await,
            },
            Ok(message) if message.is_ping() => log::debug!("ping/pong is not implemented"),
            Ok(_) => log::warn!("unsupported websocket message type"),
            Err(error) => log::error!("websocket error: {}", error),
        }
    }

    log::info!("client disconnected {:?}", connection.local.as_ref().map(|x| x.id));
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    let global = Global::default();
    let global = warp::any().map(move || global.clone());

    let ws = warp::ws()
        .and(global)
        .map(|ws: Ws, global: Global| ws.on_upgrade(|socket| start_socket(socket, global)));

    let address: SocketAddr = format!("{}:{}", opts.address, opts.port).parse().unwrap();
    warp::serve(ws).run(address).await;
}
