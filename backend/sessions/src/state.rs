use std::collections::HashMap;
use std::sync::Arc;

use rand::Rng;
use tokio::sync::{mpsc, Mutex};
use warp::ws::Message;

use sessions::{Action, Error, Request, Response};

use crate::Session;

pub type Responder = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Clone)]
pub enum Kind {
    Host,
    Manager,
}

#[derive(Clone)]
pub struct Local {
    pub id: u64,
    pub kind: Kind,
    pub state: State,
}

#[derive(Clone)]
pub struct Inner {
    session: Session,
    host: Responder,
    manager: Option<Responder>,
}

pub type State = Arc<Mutex<Inner>>;
pub type Global = Arc<Mutex<HashMap<u64, State>>>;

#[derive(Clone)]
pub struct Connection {
    pub global: Global,
    pub responder: Responder,
    pub local: Option<Local>,
}

impl Connection {
    async fn handle_host(&mut self) -> Result<Response, Error> {
        // Generate id and initialize some fields
        let id = rand::thread_rng().gen::<u32>() as u64;
        let inner =
            Inner { session: Default::default(), host: self.responder.clone(), manager: None };
        let state = Arc::new(Mutex::new(inner));

        // Set the local state for later
        self.local = Some(Local { kind: Kind::Host, state: state.clone(), id });

        // Take the lock on the sessions and insert a new one
        let mut lock = self.global.lock().await;

        match lock.insert(id, state) {
            Some(_) => Err(Error::UnableToCreate),
            None => Ok(Response { id, managed: false, session: Default::default() }),
        }
    }

    async fn handle_manage(&mut self, id: u64) -> Result<Response, Error> {
        // Take the lock on sessions to check if session with id exists
        let lock = self.global.lock().await;
        if let Some(state) = lock.get(&id) {
            // Take the local lock of the session to set it to managed
            let mut lock = state.lock().await;

            // Set the manager if non existent otherwise return error
            match lock.manager {
                Some(_) => return Err(Error::UnableToManage(id)),
                None => lock.manager = Some(self.responder.clone()),
            }

            self.local = Some(Local { kind: Kind::Manager, state: state.clone(), id });

            Ok(Response { id, managed: true, session: lock.session.clone() })
        } else {
            Err(Error::UnableToManage(id))
        }
    }

    async fn handle_update(&mut self, action: Action, rounds: usize) -> Result<Response, Error> {
        // This is already checked in the outer function
        let local = self.local.as_ref().unwrap();
        let mut lock = local.state.lock().await;

        // Apply the update if it is valid and broadcast, else raise error
        match lock.session.update(action.clone(), rounds) {
            Some(session) => {
                lock.session = session.clone();
                Ok(Response { id: local.id, managed: true, session })
            }
            None => Err(Error::InvalidUpdate(action, lock.session.clone())),
        }
    }

    pub async fn request(&mut self, request: Request) {
        let response = match (request, &self.local) {
            (Request::Host, None) => {
                log::info!("request to host a new session");
                self.handle_host().await
            }
            (Request::Manage(session_id), None) => {
                log::info!("request to manage session {}", session_id);
                self.handle_manage(session_id).await
            }
            (Request::Update(action, rounds), Some(local)) => {
                log::info!("request to update session {}", local.id);
                self.handle_update(action, rounds).await
            }
            (request, _) => {
                log::warn!("illegal request submitted {:?}", request);
                Err(Error::IllegalRequest)
            }
        };

        if let Some(_) = &self.local {
            self.respond(&response).await
        }
    }

    pub async fn respond(&self, response: &Result<Response, Error>) {
        // TODO: this is probably not safe
        let lock = self.local.as_ref().unwrap().state.lock().await;

        let str = serde_json::to_string(&response).unwrap();
        log::info!("{}", str);
        let msg = Message::text(str);

        // we don't care about the result, if the connection was closed
        // then this will error but there is no-one to report it to
        if let Some(manager) = &lock.manager {
            let _ = manager.send(Ok(msg.clone()));
        }
        let _ = lock.host.send(Ok(msg));
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        let local = match self.local.clone() {
            None => return,
            Some(local) => local,
        };

        let global = self.global.clone();

        tokio::spawn(async move {
            let mut lock = local.state.lock().await;

            match (local.kind, lock.manager.clone()) {
                (Kind::Host, Some(manager)) => {
                    let response: Result<Response, _> = Err(Error::HostDisconnected);
                    let str = serde_json::to_string(&response).unwrap();
                    manager.send(Ok(Message::text(str))).unwrap();
                    global.lock().await.remove(&local.id);
                }
                (Kind::Manager, _) => {
                    lock.manager = None;

                    let session = lock.session.clone();
                    let response: Result<_, Error> =
                        Ok(Response { id: local.id, managed: false, session });
                    let msg = Message::text(serde_json::to_string(&response).unwrap());
                    lock.host.send(Ok(msg)).unwrap();
                }
                _ => {}
            }
        });
    }
}
