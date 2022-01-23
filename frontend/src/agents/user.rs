use std::collections::HashSet;
use wasm_bindgen::JsValue;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::shared::{User, AUTH0_CLIENT_ID, AUTH0_DOMAIN};
use crate::Route;
use futures::FutureExt;
use wasm_bindgen::prelude::*;

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
    Token(Result<JsValue, JsValue>),
}

pub struct UserAgent {
    link: AgentLink<Self>,
    user: User,
    subscribers: HashSet<HandlerId>,
}

impl UserAgent {
    fn set_user(&mut self, user: User) {
        log::info!("change of user: {:?}", user);
        for id in &self.subscribers {
            self.link.respond(*id, user.clone())
        }
        self.user = user;
    }
}

impl Agent for UserAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = UserInput;
    type Output = User;

    fn create(link: AgentLink<Self>) -> Self {
        let (domain, client_id) = (AUTH0_DOMAIN.to_owned(), AUTH0_CLIENT_ID.to_owned());
        link.send_future(init_auth(domain, client_id).map(Msg::Initialized));
        Self { link, user: User::default(), subscribers: HashSet::default() }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::SignupRedirect(x) => log::info!("{:?}", x),
            Msg::LoginRedirect(x) => log::info!("{:?}", x),
            Msg::Initialized(Ok(user)) if user.is_undefined() => {
                self.set_user(User::Anonymous);
            }
            Msg::Initialized(Ok(user)) => {
                self.link.send_future(get_token().map(Msg::Token));

                match serde_wasm_bindgen::from_value(user) {
                    Ok(user) => self.set_user(User::User(user)),
                    Err(error) => log::error!("User deserialization failed: {}", error),
                };
            }
            Msg::Initialized(Err(err)) => {
                log::error!("auth error {:?}", err)
            }
            Msg::Token(Ok(token)) => {
                log::info!("{:?}", token)
            }
            Msg::Token(Err(err)) => {
                log::error!("token error {:?}", err)
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
        self.link.respond(id, self.user.clone());
    }

    fn handle_input(&mut self, input: Self::Input, id: HandlerId) {
        match input {
            UserInput::Signup => {
                self.link.send_future(redirect_to_signup().map(Msg::SignupRedirect))
            }
            UserInput::Login => self.link.send_future(redirect_to_login().map(Msg::LoginRedirect)),
            UserInput::Logout => match logout() {
                Err(error) => log::error!("logout error {:?}", error),
                Ok(_) => self.set_user(User::Anonymous),
            },
        };
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
