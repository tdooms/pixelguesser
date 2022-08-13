use std::rc::Rc;
use yew::{use_context, use_state, UseStateHandle};

use api::{Error, Login, Signup, User};
use yew::hook;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Loading,
    Anonymous,
    Authenticated { user: Rc<User>, token: Rc<String> },
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

    pub async fn login(&self, credentials: Login) -> Result<(), Error> {
        let (user, token) = api::login(credentials).await?;
        let state = State::Authenticated { user: Rc::new(user), token: Rc::new(token) };

        self.manager.state.set(state);
        Ok(())
    }
    pub async fn signup(&self, credentials: Signup) -> Result<(), Error> {
        let (user, token) = api::signup(credentials).await?;
        let state = State::Authenticated { user: Rc::new(user), token: Rc::new(token) };

        self.manager.state.set(state);
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
            State::Authenticated { token, .. } => Some(token),
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

#[hook]
pub fn use_auth_manager() -> UseAuthManagerHandle {
    let state = use_state(|| State::Anonymous); // TODO: change this once init

    // let cloned = state.clone();
    // use_startup(move || init(cloned, toast));

    UseAuthManagerHandle { state }
}
