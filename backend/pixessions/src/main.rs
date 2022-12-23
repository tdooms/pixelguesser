use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use figment::providers::{Toml, Format};
use figment::Figment;
use axum::extract::{Path, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use futures::{SinkExt, StreamExt};
use pixessions::{Mode, Session};
use rand::Rng;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};

use crate::handle::handle_session;
use crate::state::{Global, Local, State};

mod handle;
mod state;

async fn session_ws(
    ws: WebSocketUpgrade,
    ext: Extension<Global>,
    Path(session_id): Path<u32>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_session(socket, ext.0, session_id))
}

async fn create_session(ext: Extension<Global>, path: Path<(u32, Mode)>) -> String {
    let session_id = rand::thread_rng().gen::<u32>();
    let mut lock = ext.0.lock().await;

    let (quiz_id, mode) = *path;

    let state = State::new(Session::new(quiz_id, mode));
    lock.insert(session_id, Arc::new(Mutex::new(state)));

    tracing::info!("created session {session_id}");
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

#[derive(Deserialize)]
struct Config {
    address: IpAddr,
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let provider = Toml::file("config.toml").nested();
    let config: Config = Figment::from(provider).select("sessions").extract().unwrap();
    let address = SocketAddr::new(config.address, config.port);

    let global = Global::default();

    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any);

    let app = Router::new()
        .route("/ws/:session_id", get(session_ws))
        .route("/:quiz_id/:mode", post(create_session))
        .route("/", get(get_sessions))
        .layer(Extension(global))
        .layer(cors);

    tracing::info!("listening on {}", address);
    axum::Server::bind(&address).serve(app.into_make_service()).await?;

    Ok(())
}
