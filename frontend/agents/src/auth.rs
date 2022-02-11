use std::collections::HashSet;

use derive_more::Display;
use futures::FutureExt;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use keys::{AUTH0_CLIENT_ID, AUTH0_DOMAIN};
use shared::Auth;

#[derive(Clone, Debug, PartialEq, Display)]
pub enum Auth {
    #[display(fmt = "Loading")]
    Loading,
    #[display(fmt = "Anonymous")]
    Anonymous,
    #[display(fmt = "User {}", _0)]
    User(User),
}

impl From<Auth> for Option<User> {
    fn from(auth: Auth) -> Self {
        match auth {
            Auth::Loading | Auth::Anonymous => None,
            Auth::User(user) => Some(user),
        }
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

#[derive(Clone, Copy)]
pub enum UserInput {
    Login,
    Logout,
    Signup,
}

pub enum Msg {
    SignupRedirect(Result<(), JsValue>),
    LoginRedirect(Result<(), JsValue>),
    Initialized(Result<JsValue, JsValue>),
}

pub struct AuthAgent {
    link: AgentLink<Self>,
    auth: Auth,
    subscribers: HashSet<HandlerId>,
}

impl AuthAgent {
    fn set(&mut self, auth: Auth) {
        log::debug!("change of user: {}", auth);

        for id in &self.subscribers {
            self.link.respond(*id, auth.clone())
        }

        self.auth = auth;
    }
}

impl Agent for AuthAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = UserInput;
    type Output = Auth;

    fn create(link: AgentLink<Self>) -> Self {
        let (domain, client_id) = (AUTH0_DOMAIN.to_owned(), AUTH0_CLIENT_ID.to_owned());
        link.send_future(init_auth(domain, client_id).map(Msg::Initialized));

        Self { link, auth: Auth::Loading, subscribers: HashSet::default() }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SignupRedirect(Ok(())) | Msg::LoginRedirect(Ok(())) => (),
            Msg::Initialized(Ok(user)) if user.is_undefined() => {
                self.set(Auth::Anonymous);
            }
            Msg::Initialized(Ok(user)) => {
                match serde_wasm_bindgen::from_value(user) {
                    Ok(user) => self.set(Auth::User(user)),
                    Err(error) => log::error!("User deserialization failed: {}", error),
                };
            }
            Msg::Initialized(Err(err))
            | Msg::SignupRedirect(Err(err))
            | Msg::LoginRedirect(Err(err)) => {
                log::error!("user error {:?}", err)
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
        self.link.respond(id, self.auth.clone());
    }

    fn handle_input(&mut self, input: Self::Input, _: HandlerId) {
        match input {
            UserInput::Signup => {
                self.link.send_future(redirect_to_signup().map(Msg::SignupRedirect))
            }
            UserInput::Login => self.link.send_future(redirect_to_login().map(Msg::LoginRedirect)),
            UserInput::Logout => match logout() {
                Err(error) => log::error!("logout error {:?}", error),
                Ok(_) => self.set(Auth::Anonymous),
            },
        };
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
