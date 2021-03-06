use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

use api::{User, AUTH0_CLIENT_ID, AUTH0_DOMAIN};

use crate::error::Internal;
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
    pub callback: Callback<Result<User, bool>>,
}

impl Auth {
    pub fn new(callback: Callback<Result<User, bool>>) -> Self {
        let user = Rc::new(RefCell::new(Err(true)));

        let res = Self { user, callback };
        res.init();
        res
    }

    async fn init_inner() -> Result<Option<User>, Error> {
        let (domain, client_id) = (AUTH0_DOMAIN.to_owned(), AUTH0_CLIENT_ID.to_owned());
        let js_user = init_auth(domain, client_id).await.map_err(|_| Internal::AuthUnreachable)?;

        if js_user.is_undefined() {
            Ok(None)
        } else {
            let user = serde_wasm_bindgen::from_value(js_user);
            Ok(user.map(Option::Some).map_err(Internal::AuthError)?)
        }
    }

    fn init(&self) {
        let clone = self.clone();

        spawn_local(async move {
            let result = Self::init_inner().await;
            let _ = result.as_ref().map_err(|err| log::error!("auth error {err}"));

            let user = match result {
                Ok(Some(user)) => Ok(user),
                Ok(None) => Err(false),
                Err(_) => Err(true),
            };

            log::trace!("user init: {:?}", user.as_ref().map(|x| &x.nickname));

            *clone.user.borrow_mut() = user.clone();
            clone.callback.emit(user);
        });
    }

    pub fn login(&self) -> Callback<()> {
        Callback::from(|_| {
            spawn_local(async {
                let _ = redirect_to_login().await;
            })
        })
    }
    pub fn signup(&self) -> Callback<()> {
        Callback::from(|_| {
            spawn_local(async {
                let _ = redirect_to_signup().await;
            })
        })
    }
    pub fn logout(&self) -> Callback<()> {
        Callback::from(|_| {
            let _ = logout();
        })
    }
    pub fn user(&self) -> Result<User, bool> {
        self.user.borrow().clone()
    }
}
