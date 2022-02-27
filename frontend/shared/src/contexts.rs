use std::cell::RefCell;
use std::rc::Rc;

use futures::Future;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use api::{User, AUTH0_CLIENT_ID, AUTH0_DOMAIN};

use crate::Error;

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

pub type Errors = Callback<Error>;

#[derive(Clone, PartialEq)]
pub struct Auth {
    pub user: Rc<RefCell<Result<User, bool>>>,
}

impl Default for Auth {
    fn default() -> Self {
        let user = Rc::new(RefCell::new(Err(true)));

        Self { user }
    }
}

impl Auth {
    async fn init_inner() -> Result<Option<User>, Error> {
        let (domain, client_id) = (AUTH0_DOMAIN.to_owned(), AUTH0_CLIENT_ID.to_owned());
        let js_user = init_auth(domain, client_id)
            .await
            .map_err(|x| Error::AuthInit(x.as_string().unwrap_or_default()))?;

        if js_user.is_undefined() {
            Ok(None)
        } else {
            Ok(serde_wasm_bindgen::from_value(js_user).map(Option::Some)?)
        }
    }

    fn init(&self) {
        let clone = self.user.clone();

        spawn_local(async move {
            match Self::init_inner().await {
                Ok(Some(user)) => *clone.borrow_mut() = Ok(user),
                Ok(None) => *clone.borrow_mut() = Err(false),
                Err(err) => log::error!("authentication init error: {:?}", err),
            }
        })
    }

    pub fn login(&self) -> Callback<()> {
        Callback::from(|_| {
            spawn_local(async move {
                redirect_to_login();
            })
        })
    }
    pub fn signup(&self) -> Callback<()> {
        Callback::from(|_| {
            spawn_local(async move {
                redirect_to_signup();
            })
        })
    }
    pub fn logout(&self) -> Callback<()> {
        Callback::from(|_| {
            logout();
        })
    }
    pub fn user(&self) -> Result<User, bool> {
        self.user.borrow().clone()
    }
}
