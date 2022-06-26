use crate::sessions::{Global, Local, Mode, State};
use ::sessions::{Action, Error, Response};
use tower_http::cors::{Any, CorsLayer};

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{Path, WebSocketUpgrade};
use axum::http::Method;
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

mod lib;
mod sessions;

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

async fn websocket(
    ws: WebSocketUpgrade,
    Extension(global): Extension<Global>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, global, id))
}

async fn creator(Extension(global): Extension<Global>, Path(quiz): Path<u32>) -> impl IntoResponse {
    let id = rand::thread_rng().gen::<u32>();
    let mut lock = global.lock().await;

    let state = State::new(Mode::default(), quiz);
    lock.insert(id, Arc::new(Mutex::new(state)));
    log::info!("created session {id}");

    id.to_string()
}

async fn handle_message(message: Message, state: &mut State, connection: u32) -> Result<(), Error> {
    let message = message.into_text().map_err(|_| Error::NonText)?;
    let action: Action = serde_json::from_str(&message)?;

    state.session.update(action, connection);

    let response = serde_json::to_string(&Response::Update(state.session.clone())).unwrap();
    for (_, sender) in &mut state.connections {
        let _ = sender.send(Message::Text(response.clone())).await;
    }

    Ok(())
}

async fn handle_local(global: &Global, connection: u32) -> Option<Local> {
    let lock = global.lock().await;
    match lock.get(&connection) {
        Some(state) => Some(state.clone()),
        None => {
            log::error!("session not found");
            None
        }
    }
}

async fn handle_connection(stream: WebSocket, global: Global, id: u32) {
    let (sender, mut receiver) = stream.split();
    let connection = rand::thread_rng().gen::<u32>();
    log::info!("attempted connection to {id}");

    // Add the sender to the connections of the local state
    let local = match handle_local(&global, id).await {
        Some(local) => local,
        None => return,
    };

    local.lock().await.connections.insert(connection, sender);

    while let Some(Ok(message)) = receiver.next().await {
        let mut lock = local.lock().await;
        if let Err(err) = handle_message(message, &mut *lock, connection).await {
            log::error!("{}", err);
        }
    }

    // remove local connection from the session
    let mut lock = local.lock().await;
    lock.connections.remove(&connection);

    // Remove the session from the global state if it has no more connections
    if lock.connections.is_empty() {
        global.lock().await.remove(&id);
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();

    let v4 = SocketAddrV4::new(Ipv4Addr::from_str(&opts.address).unwrap(), opts.port);
    let address = SocketAddr::from(v4);

    let global = Global::default();

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([Method::GET]);

    let app = Router::new()
        .route("/ws/:id", get(websocket))
        .route("/create/:quiz", get(creator))
        .layer(Extension(global))
        .layer(cors);

    log::info!("listening on {}", address);
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}
