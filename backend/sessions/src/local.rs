use std::ops::Range;
use std::sync::{Arc, Weak};

use rand::Rng;
use sessions::{Action, Request};
use tokio::sync::Mutex;
use warp::ws::Message;

use crate::global::{Global, Responder};
use crate::{Error, Response, Session};

enum State {
    None,
    Host(u64),
    Manager(Responder, Weak<Mutex<Session>>),
}

pub struct Local {
    responder: Responder,
    state: State,
}

impl Local {
    pub fn new(responder: Responder) -> Self {
        Self { responder, state: State::None }
    }

    // This is not very rust idiomatic
    pub async fn destroy(self, global: &Global) {
        match self.state {
            State::Host(id) => {
                let mut lock = global.hosts.lock().await;
                lock.remove(&id);
            }
            State::Manager(responder, session) => {
                if let Some(session) = session.upgrade() {
                    let mut lock = session.lock().await;
                    lock.has_manager = false;

                    Local::new(responder).respond(&Response::Updated(lock.clone()))
                }
            }
            State::None => {}
        }
    }

    pub fn respond(&self, response: &Response) {
        let str = serde_json::to_string(response).unwrap();
        self.responder.send(Ok(Message::text(str))).unwrap();
    }

    async fn handle_host(&mut self, global: &Global) {
        const CHARS: u64 = 48;
        const RANGE: Range<u64> = CHARS.pow(4)..(CHARS.pow(5) - 1);

        let mut lock = global.hosts.lock().await;
        for _ in 0..20 {
            let id = rand::thread_rng().gen_range(RANGE.clone());

            if !lock.contains_key(&id) {
                lock.insert(id, (self.responder.clone(), Arc::default()));
                self.state = State::Host(id)
            }
        }
    }

    async fn handle_manage(&mut self, session_id: u64, global: &Global) {
        // Get a lock on the global data to access to session
        let lock = global.hosts.lock().await;
        match lock.get(&session_id) {
            Some((responder, session)) => {
                // Lock the session and check if it is already manager
                let mut lock = session.lock().await;
                if lock.has_manager {
                    self.respond(&Response::Error(Error::UnableToManage(session_id)));
                    return;
                }

                // Otherwise set the managed flag and broadcast the update
                lock.has_manager = true;
                let update = Response::Updated(lock.clone());
                Local::new(responder.clone()).respond(&update);
                self.state = State::Manager(responder.clone(), Arc::downgrade(session))
            }
            _ => self.respond(&Response::Error(Error::UnableToManage(session_id))),
        }
    }

    async fn handle_update(
        &self,
        action: Action,
        rounds: u64,
        host: &Responder,
        session: &Weak<Mutex<Session>>,
    ) {
        match session.upgrade() {
            Some(session) => {
                let mut lock = session.lock().await;
                let updated = match lock.update(action.clone(), rounds) {
                    None => Response::Error(Error::InvalidUpdate(action, lock.clone())),
                    Some(session) => {
                        *lock = session.clone();
                        Response::Updated(session)
                    }
                };
                Local::new(host.clone()).respond(&updated);
                self.respond(&updated);
            }
            None => {
                log::warn!("host disconnected")
            }
        }
    }

    pub async fn request(&mut self, request: Request, global: &Global) {
        match (request, &self.state) {
            (Request::Host, State::None) => {
                log::debug!("attempting to host new session");
                self.handle_host(global).await
            }
            (Request::Manage(session_id), State::None) => {
                log::debug!("attempting to manage session");
                self.handle_manage(session_id, global).await
            }
            (Request::Update(action, rounds), State::Manager(host, session)) => {
                log::debug!("attempting to update session");
                self.handle_update(action, rounds, host, session).await
            }
            (_, State::None) => {
                log::warn!("must first manage a session before submitting actions")
            }
            (_, State::Host(_)) => {
                log::warn!("cannot submit requests as a host")
            }
            _ => {
                log::warn!("forbidden operation")
            }
        }
    }
}
