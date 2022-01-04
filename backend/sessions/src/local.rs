use crate::global::{Global, Responder};
use crate::{Error, Response, Session};
use rand::Rng;
use sessions::{Action, Request};
use std::sync::{Arc, Weak};
use tokio::sync::Mutex;
use warp::ws::Message;

enum State {
    None,
    Host(u64),
    Manager(Responder, Weak<Mutex<Session>>),
}

enum Answer {
    Both(Response),
    Different { host: Response, manager: Response },
    Back(Response),
    None,
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

    async fn handle_host(&mut self, global: &Global) -> Answer {
        let mut lock = global.hosts.lock().await;
        let id = rand::thread_rng().gen::<u32>() as u64;
        self.state = State::Host(id);

        match lock.insert(id, (self.responder.clone(), Arc::default())) {
            None => Answer::Back(Response::Hosted(id, Session::default())),
            Some(_) => Answer::Back(Response::Error(Error::UnableToCreate)),
        }
    }

    async fn handle_manage(&mut self, session_id: u64, global: &Global) -> Answer {
        // Get a lock on the global data to access to session
        let lock = global.hosts.lock().await;
        match lock.get(&session_id) {
            Some((responder, session)) => {
                // Lock the session and check if it is already manager
                let mut lock = session.lock().await;
                if lock.has_manager {
                    return Answer::Back(Response::Error(Error::UnableToManage(session_id)));
                }

                // Otherwise set the managed flag and broadcast the update
                lock.has_manager = true;
                self.state = State::Manager(responder.clone(), Arc::downgrade(session));

                let host = Response::Updated(lock.clone());
                let manager = Response::Managed(session_id, lock.clone());
                Answer::Different { host, manager }
            }
            _ => Answer::Back(Response::Error(Error::UnableToManage(session_id))),
        }
    }

    async fn handle_update(
        &self,
        action: Action,
        rounds: usize,
        session: &Weak<Mutex<Session>>,
    ) -> Answer {
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
                Answer::Both(updated)
            }
            None => {
                log::warn!("host disconnected");
                Answer::Back(Response::Error(Error::HostDisconnected))
            }
        }
    }

    pub async fn request(&mut self, request: Request, global: &Global) {
        let answer = match (request, &self.state) {
            (Request::Host, State::None) => {
                log::debug!("attempting to host new session");
                self.handle_host(global).await
            }
            (Request::Manage(session_id), State::None) => {
                log::debug!("attempting to manage session");
                self.handle_manage(session_id, global).await
            }
            (Request::Update(action, rounds), State::Manager(_, session)) => {
                log::debug!("attempting to update session");
                self.handle_update(action, rounds, session).await
            }
            (_, State::None) => {
                log::warn!("must first manage a session before submitting actions");
                Answer::None
            }
            (_, State::Host(_)) => {
                log::warn!("cannot submit requests as a host");
                Answer::None
            }
            _ => {
                log::warn!("forbidden operation");
                Answer::None
            }
        };
        match (answer, &self.state) {
            (Answer::Both(response), State::Manager(resp, _)) => {
                self.respond(&response);
                Local::new(resp.clone()).respond(&response)
            }
            (Answer::Different { host, manager }, State::Manager(resp, _)) => {
                self.respond(&manager);
                Local::new(resp.clone()).respond(&host)
            }
            (Answer::Back(response), _) => self.respond(&response),
            (Answer::None, _) => {}
            _ => log::warn!("wrong answer for given local state"),
        }
    }
}
