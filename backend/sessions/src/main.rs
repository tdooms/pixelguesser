use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

use clap::{AppSettings, Clap};
use futures::StreamExt;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{Error, Filter};
use warp::ws::{Message, WebSocket, Ws};

use shared::{Player, Request, Response, Session, Stage};
use rand::Rng;
use std::collections::hash_map::Entry;

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

fn send_message(sender: &Sender, response: &Response) {
    let str = serde_json::to_string(response).unwrap();
    sender.send(Ok(Message::text(str))).unwrap()
}

async fn manipulate_session(sender: &Sender, session_id: u64, state: &State, mapper: impl FnOnce(&SessionData) -> Response) {
    let response = match state.lock().await.get(&session_id) {
        None => Response::Error("no session with id".to_owned()),
        Some(data) => mapper(data)
    };
    send_message(sender, &response)
}

async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    match serde_json::from_slice(request) {
        Ok(Request::Read(session_id)) => {
            let response = match state.lock().await.get(&session_id) {
                None => Response::Error("no session with id".to_owned()),
                Some(SessionData { session, .. }) => Response::Read(session.clone())
            };
            send_message(sender, &response);
        }
        Ok(Request::Check(session_id)) => {
            let response = match state.lock().await.get(&session_id) {
                None => Response::Checked(None),
                Some(SessionData { manager: Some(_), .. }) => Response::Checked(None),
                _ => Response::Checked(Some(session_id)),
            };
            send_message(sender, &response);
        }
        Ok(Request::Update(session_id, session)) => {
            match state.lock().await.get_mut(&session_id) {
                None => {
                    send_message(sender, &Response::Error("no session with id".to_owned()));
                }
                Some(data) => {
                    data.session = session.clone();
                    let response = Response::Updated(session);

                    data.manager.as_ref().map(|manager| send_message(manager, &response));
                    data.host.as_ref().map(|host| send_message(host, &response));
                }
            };
        }
        Ok(Request::Create) => {
            static SESSION_MAX: u64 = 48u64.pow(6);
            let mut lock = state.lock().await;

            for _ in 0..10 {
                let id = rand::thread_rng().gen_range(0..SESSION_MAX);

                // insert if available, continue if not
                match lock.entry(id) {
                    Entry::Occupied(_) => continue,
                    Entry::Vacant(entry) => entry.insert(SessionData::default())
                }

                // send message if inserted and return from function to prevent further messages
                send_message(sender, &Response::Created(id));
                return
            }
            send_message(sender, &Response::Error("could not make session".to_owned()));
        },
        Ok(Request::Host(session_id)) => {
            let response = match state.lock().await.get(&session_id) {
                None => Response::Error("no session with id".to_owned()),
                Some(SessionData { session, .. }) => Response::Read(session.clone())
            };
            send_message(sender, &response);
        }
        Ok(Request::Manage(session_id)) => {
            let response = match state.lock().await.get(&session_id) {
                None => Response::Error("no session with id".to_owned()),
                Some(SessionData { manager, .. }) => Response::Read(session.clone())
            };
            send_message(sender, &response);
        }
        Err(err) => {
            log!("{:?}", err);
            send_message(sender, &Response::Error("deserialize error".to_owned()));
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
