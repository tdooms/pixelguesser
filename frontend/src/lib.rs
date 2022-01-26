use cobul::props::Color;
use cobul::Notification;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::agents::{ErrorAgent, UserAgent};
use crate::components::Loader;
use crate::pages::*;
use crate::shared::{Error, Route, User};
use crate::utils::string_to_code;
use std::rc::Rc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod agents;
mod components;
mod graphql;
mod pages;
mod shared;
mod utils;

pub struct Model {
    _user_agent: Box<dyn Bridge<UserAgent>>,
    _error_agent: Box<dyn Bridge<ErrorAgent>>,

    user: Option<User>,
    error: Option<Rc<Error>>,
}

pub enum Msg {
    User(Option<User>),
    Error(Rc<Error>),
    Remove,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _user_agent: UserAgent::bridge(ctx.link().callback(Msg::User)),
            _error_agent: ErrorAgent::bridge(ctx.link().callback(Msg::Error)),
            user: None,
            error: None,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::User(user) => self.user = user,
            Msg::Error(error) => self.error = Some(error),
            Msg::Remove => self.error = None,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inner = match self.user.clone() {
            Some(user) => html! {
                <ContextProvider<User> context={user}>
                    <Switch<Route> render={Switch::render(switch)} />
                </ContextProvider<User>>
            },
            None => html! {
                <Switch<Route> render={Switch::render(switch)} />
            },
        };

        let notification = match &self.error {
            None => html! {},
            Some(error) => html! {
                <Notification color={Color::Danger} light=true onclick={ctx.link().callback(|_| Msg::Remove)}>
                    { format!("{}", error.clone()) }
                </Notification>
            },
        };

        html! {
            <main>
                <BrowserRouter>
                    { notification }
                    { inner }
                </BrowserRouter>
            </main>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Host { quiz_id } => {
            html! { <Loader quiz_id={*quiz_id}/> }
        }
        Route::Manage { code } => {
            let (session_id, quiz_id) = string_to_code(code).unwrap();
            html! { <Loader quiz_id={quiz_id} session_id={session_id}/> }
        }
        Route::Create => html! { <Create/> },
        Route::Update { quiz_id } => html! { <Create quiz_id={*quiz_id}/> },
        Route::Test => html! { <> {"test"} </> },
        Route::Overview => html! { <Overview/> },
        Route::NotFound => html! { <Overview/> },
    }
}

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
    Ok(())
}
