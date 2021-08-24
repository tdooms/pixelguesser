use std::collections::hash_map::Entry;
use std::ops::Range;

use rand::Rng;
use warp::ws::Message;

use shared::{Error, Request, Response, Session, SessionDiff};

use crate::structs::{InternalSession, Sender, State};

fn into_session(session_id: u64, internal: &InternalSession) -> Session {
    Session {
        session_id,
        quiz_id: internal.quiz_id,
        stage: internal.stage,
        players: internal.players.clone(),
        has_manager: internal.manager.is_some(),
        has_host: internal.host.is_some(),
    }
}

fn combine(internal: &mut InternalSession, diff: SessionDiff) {
    if let Some(x) = diff.players {
        internal.players = x;
    }
    internal.stage = diff.stage.unwrap_or(internal.stage);
}

fn send_response(sender: &Sender, response: &Response) {
    let str = serde_json::to_string(response).unwrap();
    sender.send(Ok(Message::text(str))).unwrap()
}

fn broadcast_update(session_id: u64, internal: &InternalSession) {
    let response = Response::Updated(into_session(session_id, internal));
    internal.manager.as_ref().map(|manager| send_response(manager, &response));
    internal.host.as_ref().map(|host| send_response(host, &response));
}

fn maybe_insert_sender(option: &mut Option<Sender>, sender: &Sender) -> bool {
    let ret = option.is_none();
    *option = option.as_ref().or_else(|| Some(sender)).cloned();
    ret
}

async fn create(state: &State, quiz_id: u64) -> Response {
    static RANGE: Range<u64> = 0..48u64.pow(6);
    let mut lock = state.lock().await;

    for _ in 0..10 {
        let id = rand::thread_rng().gen_range(RANGE.clone());

        match lock.entry(id) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(entry) => {
                let internal = InternalSession {
                    quiz_id,
                    host: None,
                    manager: None,
                    players: vec![],
                    stage: Default::default(),
                };

                let session = into_session(id, &internal);
                entry.insert(internal);
                return Response::Created(session);
            }
        }
    }
    Response::Error(Error::UnableToCreate)
}

pub enum Kind {
    Read,
    Update(SessionDiff),
    Host,
    Manage,
}

pub async fn handle_request(request: &[u8], state: &State, sender: &Sender) {
    let (session_id, kind) = match serde_json::from_slice(request) {
        Ok(Request::Read { session_id }) => (session_id, Kind::Read),
        Ok(Request::Update { session_id, diff }) => (session_id, Kind::Update(diff)),
        Ok(Request::Host { session_id }) => (session_id, Kind::Host),
        Ok(Request::Manage { session_id }) => (session_id, Kind::Manage),
        Ok(Request::Create { quiz_id }) => {
            return send_response(sender, &create(state, quiz_id).await);
        }
        Err(_) => {
            return send_response(sender, &Response::Error(Error::UnknownRequest));
        }
    };

    let mut lock = state.lock().await;
    let internal = match lock.get_mut(&session_id) {
        None => return send_response(sender, &Response::Error(Error::SessionDoesNotExist(session_id))),
        Some(x) => x
    };

    match kind {
        Kind::Read => {
            let response = Response::Read(into_session(session_id, internal));
            send_response(sender, &response);
        },
        Kind::Update(diff) => {
            combine(internal, diff);
            broadcast_update(session_id, &internal);
        }
        Kind::Host => {
            if maybe_insert_sender(&mut internal.host, sender){
                broadcast_update(session_id, internal)
            }
        }
        Kind::Manage => {
            log::info!("{:?}", internal);
            if maybe_insert_sender(&mut internal.manager, sender){
                broadcast_update(session_id, internal)
            }
            log::info!("{:?}", internal);
        }
    };
}