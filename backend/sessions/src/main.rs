use std::collections::HashMap;
use std::sync::Arc;

use clap::{AppSettings, Clap};
use futures::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Filter};
use warp::ws::{Message, WebSocket, Ws};

use shared::{Request, Response, Session, Error};
use rand::Rng;
use std::collections::hash_map::Entry;
use std::net::SocketAddr;
use std::ops::Range;

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

#[derive(Clone, Debug, Default)]
pub struct SessionData {
    pub host: Option<Sender>,
    pub manager: Option<Sender>,

    pub session: Session,
}

pub type State = Arc<Mutex<HashMap<u64, SessionData>>>;

fn send_response(sender: &Sender, response: &Response) {
    let str = serde_json::to_string(response).unwrap();
    sender.send(Ok(Message::text(str))).unwrap()
}

fn broadcast_update(data: &SessionData) {
    let response = Response::Updated(data.session.clone());
    data.manager.as_ref().map(|manager| send_response(manager, &response));
    data.host.as_ref().map(|host| send_response(host, &response));
}

async fn create(state: &State) -> Response {
    static RANGE: Range<u64> = 0..48u64.pow(6);
    let mut lock = state.lock().await;

    for _ in 0..10 {
        let id = rand::thread_rng().gen_range(RANGE.clone());

        match lock.entry(id) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => { entry.insert(SessionData::default()); }
        }

        return Response::Created(id);
    }
    Response::Error(Error::UnableToCreate)
}

async fn find_and_read(sender: &Sender, state: &State, session_id: u64) {
    let response = match state.lock().await.get_mut(&session_id) {
        None => Response::Error(Error::SessionDoesNotExist(session_id)),
        Some(data) => Response::Read(data.session.clone())
    };
    send_response(sender, &response)
}

async fn find_and_broadcast_update(sender: &Sender, state: &State, session_id: u64, mapper: impl FnOnce(&mut SessionData)) {
    match state.lock().await.get_mut(&session_id) {
        None => send_response(sender, &Response::Error(Error::SessionDoesNotExist(session_id))),
        Some(data) => {
            mapper(data);
            broadcast_update(data)
        }
    }
}

async fn find_and_broadcast_or_error(sender: &Sender, state: &State, session_id: u64, mapper: impl FnOnce(&mut SessionData) -> bool, error: Error) {
    match state.lock().await.get_mut(&session_id) {
        None => send_response(sender, &Response::Error(Error::SessionDoesNotExist(session_id))),
        Some(data) => match mapper(data) {
            true => send_response(sender, &Response::Error(error)),
            false => broadcast_update(data),
        }
    }
}

fn maybe_insert_sender(option: &mut Option<Sender>, sender: &Sender) -> bool {
    let ret = option.is_some();
    *option = option.as_ref().or_else(|| Some(sender)).cloned();
    ret
}

async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    match serde_json::from_slice(request) {
        Ok(Request::Read(session_id)) => {
            find_and_read(sender, state, session_id).await;
        }
        Ok(Request::Update(session_id, session)) => {
            let mapper = |data: &mut SessionData| data.session = session.clone();
            find_and_broadcast_update(sender, state, session_id, mapper).await;
        }
        Ok(Request::Create) => {
            send_response(sender, &create(state).await)
        }
        Ok(Request::Host(session_id)) => {
            let mapper = |data: &mut SessionData| maybe_insert_sender(&mut data.host, sender);
            find_and_broadcast_or_error(sender, state, session_id, mapper, Error::UnableToHost(session_id)).await;
        }
        Ok(Request::Manage(session_id)) => {
            let mapper = |data: &mut SessionData| maybe_insert_sender(&mut data.host, sender);
            find_and_broadcast_or_error(sender, state, session_id, mapper, Error::UnableToManage(session_id)).await;
        }
        Ok(_) => todo!("handle unhost and disconnects"),
        Err(err) => {
            log::warn!("{:?}", err);
            send_response(sender, &Response::Error(Error::UnknownRequest));
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

    let address: SocketAddr = format!("{}:{}", opts.address, opts.port).parse().unwrap();
    warp::serve(ws).run(address).await;
}
