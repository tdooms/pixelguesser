use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew::{use_context, use_state, UseStateHandle};

use crate::{use_startup, Kind};
use api::{User, AUTH0_CLIENT_ID, AUTH0_DOMAIN};
use yew::hook;

use crate::Toast;

#[derive(derive_more::Display)]
#[display(fmt = "Something went wrong with the authentication")]
pub enum AuthError {
    Unreachable,
    Error,
}

impl Toast for AuthError {
    fn kind(&self) -> Kind {
        Kind::Error
    }
    fn leave(&self) -> bool {
        true
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn init_auth(domain: String, client_id: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn redirect_to_signup() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    async fn redirect_to_login() -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    async fn get_token() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    fn logout() -> Result<(), JsValue>;
}

#[derive(PartialEq, Debug, Clone)]
pub enum AuthState {
    Loading,
    Anonymous,
    Authenticated(Rc<User>),
}

#[derive(Clone, PartialEq)]
pub struct UseAuthHandle {
    manager: UseAuthManagerHandle,
}

impl UseAuthHandle {
    pub fn user(&self) -> Option<Rc<User>> {
        self.manager.user()
    }
    pub fn loading(&self) -> bool {
        self.manager.loading()
    }

    pub fn login(&self) {
        ywt::spawn!(async { redirect_to_login().await.unwrap() })
    }
    pub fn signup(&self) {
        ywt::spawn!(async { redirect_to_signup().await.unwrap() })
    }
    pub fn logout(&self) {
        self.manager.state.set(AuthState::Anonymous);
        logout().unwrap();
    }
}

#[derive(Clone, PartialEq)]
pub struct UseAuthManagerHandle {
    state: UseStateHandle<AuthState>,
}

impl UseAuthManagerHandle {
    pub fn user(&self) -> Option<Rc<User>> {
        match (*self.state).clone() {
            AuthState::Authenticated(user) => Some(user),
            _ => None,
        }
    }

    pub fn loading(&self) -> bool {
        (*self.state).clone() == AuthState::Loading
    }
}

#[hook]
pub fn use_auth() -> UseAuthHandle {
    let manager = use_context::<UseAuthManagerHandle>().expect("auth context must be defined");
    UseAuthHandle { manager }
}

fn init(state: UseStateHandle<AuthState>) {
    ywt::spawn!(async move {
        // Unreachable
        let result = init_auth(AUTH0_DOMAIN.to_owned(), AUTH0_CLIENT_ID.to_owned()).await.unwrap();

        // Auth Error
        match serde_wasm_bindgen::from_value::<User>(result) {
            Ok(user) => {
                log::info!("{}", user.token.clone());
                state.set(AuthState::Authenticated(Rc::new(user)))
            }
            Err(_) => (),
        }
    });
}

#[hook]
pub fn use_auth_manager() -> UseAuthManagerHandle {
    let state = use_state(|| AuthState::Anonymous);
    let cloned = state.clone();

    use_startup(move || init(cloned));

    UseAuthManagerHandle { state }
}
