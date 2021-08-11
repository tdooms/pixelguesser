use std::collections::HashMap;
use std::sync::Arc;

use clap::{AppSettings, Clap};
use futures::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Error, Filter};
use warp::ws::{Message, WebSocket, Ws};

use shared::{Player, Request, Session, Stage};
use std::net::{Ipv4Addr, IpAddr};

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

pub type Sender = mpsc::UnboundedSender<Result<Message, warp::Error>>;

pub struct SessionData {
    pub host: Option<Sender>,
    pub manager: Option<Sender>,

    pub session: Session,
}

pub type State = Arc<Mutex<HashMap<u64, SessionData>>>;

async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    match serde_json::from_slice(request) {
        Ok(Request::Read(session_id)) => {
            let lock = state.lock().await;

            match lock.get(&session_id) {
                None => {}
                Some(SessionData { session, .. }) => {
                    sender.send()
                }
            }
        }
    }
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

    log::debug!("client disconnected");
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let state = warp::any().map(move || State::default());

    let ws = warp::path("ws")
        .and(warp::ws())
        .and(state)
        .map(|ws: Ws, state: State| ws.on_upgrade(|socket| start_socket(socket, state)));

    warp::serve(ws).run(format!("{}:{}", opts.address, opts.port)).await;
}
