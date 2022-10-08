use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::{Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use clap::Parser;
use futures::{SinkExt, StreamExt};
use pixessions::{Mode, Session};
use rand::Rng;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::handle::handle_session;
use crate::state::{Global, Local, State};

mod handle;
mod lib;
mod state;

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

async fn session_ws(
    ws: WebSocketUpgrade,
    ext: Extension<Global>,
    Path(session_id): Path<u32>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_session(socket, ext.0, session_id))
}

async fn create_session(ext: Extension<Global>, Path(quiz_id): Path<u32>) -> String {
    let session_id = rand::thread_rng().gen::<u32>();
    let mut lock = ext.0.lock().await;

    let state = State::new(Session::new(quiz_id, Mode::default()));
    lock.insert(session_id, Arc::new(Mutex::new(state)));

    log::info!("created session {session_id}");
    session_id.to_string()
}

async fn get_sessions(ext: Extension<Global>) -> Json<Vec<Session>> {
    let mut sessions = Vec::new();
    for session in ext.0.lock().await.values() {
        let session = session.lock().await.session.clone();
        sessions.push(session);
    }
    Json(sessions)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let opts: Opts = Opts::parse();

    let v4 = SocketAddrV4::new(Ipv4Addr::from_str(&opts.address).unwrap(), opts.port);
    let address = SocketAddr::from(v4);

    let global = Global::default();

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app = Router::new()
        .route("/ws/:session_id", get(session_ws))
        .route("/:quiz_id", post(create_session))
        .route("/", get(get_sessions))
        .layer(Extension(global))
        .layer(cors);

    log::info!("listening on {}", address);
    axum::Server::bind(&address).serve(app.into_make_service()).await.unwrap();
}
