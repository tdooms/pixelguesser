use crate::sessions::{Global, Mode, Session};
use crate::shared::Action;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use rand::Rng;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;

mod sessions;
mod shared;

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

async fn handler(
    ws: WebSocketUpgrade,
    Extension(global): Extension<Global>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, global, id))
}

async fn creator(Extension(global): Extension<Global>, Path(quiz): Path<u64>) -> impl IntoResponse {
    let id = rand::thread_rng().gen::<u64>();
    let mut lock = global.sessions.lock().await;

    let session = Session::new(Mode::default(), quiz);
    lock.insert(id, Arc::new(Mutex::new(session)));

    id.to_string()
}

async fn websocket(stream: WebSocket, global: Global, id: u64) {
    let (sender, mut receiver) = stream.split();
    let connection = rand::thread_rng().gen::<u64>();

    // Add the sender to the connections of the local state
    let local = {
        let lock = global.sessions.lock().await;
        lock.get(&id).unwrap().clone()
    };
    local.lock().await.connections.insert(connection, sender);

    while let Some(Ok(message)) = receiver.next().await {
        // Parse the message
        let message = message.into_text().unwrap();
        let action: Action = serde_json::from_str(&message).unwrap();

        // Update the local session state
        let mut lock = local.lock().await;
        lock.state.update(action);

        // Notify all the listeners of the updated state
        let response = serde_json::to_string(&lock.state).unwrap();
        for (_, sender) in &mut lock.connections {
            let _ = sender.send(Message::Text(response.clone())).await;
        }
    }

    // remove local connection from the session
    let mut lock = local.lock().await;
    lock.connections.remove(&connection);

    // Remove the session from the global state if it has no more connections
    if lock.connections.is_empty() {
        global.sessions.lock().await.remove(&id);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();

    let v4 = SocketAddrV4::new(Ipv4Addr::from_str(&opts.address).unwrap(), opts.port);
    let address = SocketAddr::from(v4);

    let global = Global::default();

    let app = Router::new()
        .route("ws/:id", get(handler))
        .layer(Extension(global))
        .route("create/:quiz", get(creator));

    log::debug!("listening on {}", address);
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}
