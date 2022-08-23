use gloo::storage::{LocalStorage, Storage};
use std::rc::Rc;
use yew::{use_context, use_state, UseStateHandle};

use crate::use_startup;
use api::{create_user, query_user, Claims, Credentials, Error, Response, Tokens, User};
use yew::hook;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Loading,
    Anonymous,
    Partial { id: Rc<String>, bearer: Rc<String> },
    Authenticated { user: Rc<User>, bearer: Rc<String> },
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
        let tokens = api::login(credentials).await?;
        LocalStorage::set("pixauth", &tokens).unwrap();

        let bearer = Rc::new(format!("Bearer {}", tokens.bearer));

        let state = match query_user(Some(bearer.clone()), tokens.id.clone()).await? {
            Some(user) => State::Authenticated { user: Rc::new(user), bearer },
            None => State::Partial { id: Rc::new(tokens.id), bearer },
        };

        self.manager.state.set(state);
        Ok(())
    }

    pub async fn profile(&self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn signup(&self, credentials: Rc<Credentials>) -> Result<(), Error> {
        let tokens = api::signup(credentials).await?;
        LocalStorage::set("pixauth", &tokens).unwrap();

        let bearer = Rc::new(format!("Bearer {}", tokens.bearer));

        self.manager.state.set(State::Partial { id: Rc::new(tokens.id), bearer });
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
            State::Authenticated { bearer, .. } | State::Partial { bearer, .. } => Some(bearer),
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
    let tokens: Result<Tokens, _> = LocalStorage::get("pixauth");
    let new = if let Ok(tokens) = tokens {
        let bearer = Rc::new(tokens.bearer);
        State::Partial { id: Rc::new(tokens.id), bearer }
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
