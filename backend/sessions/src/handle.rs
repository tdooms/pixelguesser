use std::collections::hash_map::Entry;
use std::ops::Range;

use rand::Rng;

use warp::ws::Message;

use shared::{Error, Request, Response, Session};

use crate::structs::{Sender, SessionData, State};

fn send_response(sender: &Sender, response: &Response) {
    let str = serde_json::to_string(response).unwrap();
    sender.send(Ok(Message::text(str))).unwrap()
}

fn broadcast_update(data: &SessionData) {
    let response = Response::Updated(data.session.clone());
    data.manager.as_ref().map(|manager| send_response(manager, &response));
    data.host.as_ref().map(|host| send_response(host, &response));
}

async fn create(state: &State, quiz_id: u64) -> Response {
    static RANGE: Range<u64> = 0..48u64.pow(6);
    let mut lock = state.lock().await;

    for _ in 0..10 {
        let id = rand::thread_rng().gen_range(RANGE.clone());

        match lock.entry(id) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                let session = Session {
                    quiz_id,
                    stage: Default::default(),
                    players: vec![],
                    has_manager: false,
                    has_host: false,
                };

                entry.insert(SessionData { manager: None, host: None, session: session.clone() });
                return Response::Created(id, session);
            }
        }
    }
    Response::Error(Error::UnableToCreate)
}

async fn find_and_read(sender: &Sender, state: &State, session_id: u64) {
    let response = match state.lock().await.get_mut(&session_id) {
        None => Response::Error(Error::SessionDoesNotExist(session_id)),
        Some(data) => Response::Read(data.session.clone()),
    };
    send_response(sender, &response)
}

async fn find_and_broadcast_update(
    sender: &Sender,
    state: &State,
    session_id: u64,
    mapper: impl FnOnce(&mut SessionData),
) {
    match state.lock().await.get_mut(&session_id) {
        None => send_response(sender, &Response::Error(Error::SessionDoesNotExist(session_id))),
        Some(data) => {
            mapper(data);
            broadcast_update(data)
        }
    }
}

async fn find_and_broadcast_or_error(
    sender: &Sender,
    state: &State,
    session_id: u64,
    mapper: impl FnOnce(&mut SessionData) -> bool,
    error: Error,
) {
    match state.lock().await.get_mut(&session_id) {
        None => send_response(sender, &Response::Error(Error::SessionDoesNotExist(session_id))),
        Some(data) => match mapper(data) {
            true => send_response(sender, &Response::Error(error)),
            false => broadcast_update(data),
        },
    }
}

fn maybe_insert_sender(option: &mut Option<Sender>, sender: &Sender) -> bool {
    let ret = option.is_some();
    *option = option.as_ref().or_else(|| Some(sender)).cloned();
    ret
}

pub async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    match serde_json::from_slice(request) {
        Ok(Request::Read { session_id }) => {
            find_and_read(sender, state, session_id).await;
        }
        Ok(Request::Update { session_id, session }) => {
            let mapper = |data: &mut SessionData| data.session = session.clone();
            find_and_broadcast_update(sender, state, session_id, mapper).await;
        }
        Ok(Request::Create { quiz_id }) => send_response(sender, &create(state, quiz_id).await),
        Ok(Request::Host { session_id }) => {
            let mapper = |data: &mut SessionData| maybe_insert_sender(&mut data.host, sender);
            find_and_broadcast_or_error(
                sender,
                state,
                session_id,
                mapper,
                Error::UnableToHost(session_id),
            )
            .await;
        }
        Ok(Request::Manage { session_id }) => {
            let mapper = |data: &mut SessionData| maybe_insert_sender(&mut data.host, sender);
            find_and_broadcast_or_error(
                sender,
                state,
                session_id,
                mapper,
                Error::UnableToManage(session_id),
            )
            .await;
        }
        Err(err) => {
            log::warn!("{:?}", err);
            send_response(sender, &Response::Error(Error::UnknownRequest));
        }
    }
}
