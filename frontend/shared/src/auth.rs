use gloo::storage::{LocalStorage, Storage};
use std::rc::Rc;
use yew::{use_context, use_state, UseStateHandle};

use crate::use_startup;
use api::{create_user, query_user, Credentials, Error, Response, Tokens, User};
use jsonwebtoken::*;
use yew::hook;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Loading,
    Anonymous,
    Partial { id: Rc<String>, bearer: Rc<String> },
    Authenticated { user: Rc<User>, bearer: Rc<String> },
}

fn decode_bearer(response: &str) {
    let secret = "1234567890123456789012345678901234567890"; // Shhh this is secret
    let key = DecodingKey::from_secret(secret.as_bytes());
}

#[derive(Clone, PartialEq)]
pub struct UseAuthHandle {
    manager: UseAuthManagerHandle,
}

impl UseAuthHandle {
    pub fn user(&self) -> Option<Rc<User>> {
        self.manager.user()
    }
    pub fn token(&self) -> Option<Rc<String>> {
        self.manager.token()
    }
    pub fn loading(&self) -> bool {
        self.manager.loading()
    }

    pub async fn login(&self, credentials: Rc<Credentials>) -> Result<(), Error> {
        let response = api::login(credentials).await?;
        LocalStorage::set("pixauth", &response).unwrap();

        let token = Rc::new(format!("Bearer {}", response.bearer));

        let state = match query_user(Some(token.clone()), response.id.clone()).await? {
            Some(user) => State::Authenticated { user: Rc::new(user), bearer: token },
            None => State::Partial { id: Rc::new(response.id), bearer: token },
        };

        self.manager.state.set(state);
        Ok(())
    }

    pub async fn profile(&self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn signup(&self, credentials: Rc<Credentials>) -> Result<(), Error> {
        let response = api::signup(credentials).await?;

        let token = Rc::new(format!("Bearer {}", response.token));
        self.manager.state.set(State::Partial { id: Rc::new(response.id), bearer: token });
        Ok(())
    }
    pub fn logout(&self) {
        self.manager.state.set(State::Anonymous);
    }
}

#[derive(Clone, PartialEq)]
pub struct UseAuthManagerHandle {
    state: UseStateHandle<State>,
}

impl UseAuthManagerHandle {
    pub fn user(&self) -> Option<Rc<User>> {
        match (*self.state).clone() {
            State::Authenticated { user, .. } => Some(user),
            _ => None,
        }
    }

    pub fn token(&self) -> Option<Rc<String>> {
        match (*self.state).clone() {
            State::Authenticated { bearer: token, .. } | State::Partial { bearer: token, .. } => {
                Some(token)
            }
            _ => None,
        }
    }

    pub fn loading(&self) -> bool {
        (*self.state).clone() == State::Loading
    }
}

#[hook]
pub fn use_auth() -> UseAuthHandle {
    let manager = use_context::<UseAuthManagerHandle>().expect("auth context must be defined");
    UseAuthHandle { manager }
}

fn init(state: UseStateHandle<State>) {
    let response: Result<Tokens, _> = LocalStorage::get("pixauth");
    let new = if let Ok(response) = response {
        let id = decode(bearer, &key, &Validation::default()).unwrap();

        let bearer = Rc::new(response.bearer);
        State::Partial { id, bearer }
    } else {
        State::Anonymous
    };

    state.set(new)
}

#[hook]
pub fn use_auth_manager() -> UseAuthManagerHandle {
    let state = use_state(|| State::Loading);
    log::info!("{:?}", *state);

    let cloned = state.clone();
    use_startup(move || init(cloned));

    UseAuthManagerHandle { state }
}
