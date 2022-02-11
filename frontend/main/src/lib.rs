mod loader;
mod overview;

use cobul::props::Color;
use cobul::{Loading, Notification};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use crate::agents::{ErrorAgent, UserAgent};
use crate::components::Loader;
use crate::pages::*;
use crate::shared::{Auth, Error, Route};
use api::string_to_code;
use std::rc::Rc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Model {
    _user_agent: Box<dyn Bridge<AuthAgent>>,
    _error_agent: Box<dyn Bridge<ErrorAgent>>,

    auth: Auth,
    error: Option<Rc<Error>>,
}

#[derive(Debug)]
pub enum Msg {
    Auth(Auth),
    Error(Rc<Error>),
    Remove,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _user_agent: UserAgent::bridge(ctx.link().callback(Msg::Auth)),
            _error_agent: ErrorAgent::bridge(ctx.link().callback(Msg::Error)),
            auth: Auth::Loading,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Auth(auth) => self.auth = auth,
            Msg::Error(error) => {
                match &*error {
                    Error::Session(_) => ctx.link().history().unwrap().push(Route::Overview),
                    _ => {}
                }
                self.error = Some(error);
            }
            Msg::Remove => self.error = None,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let inner = match &self.auth {
            Auth::Loading => html! { <Loading />},
            _ => html! { <Switch<Route> render={Switch::render(switch)} /> },
        };

        let notification = match &self.error {
            None => html! {},
            Some(error) => html! {
                <div style="position:absolute; top:55px; left:30px; z-index: 10">
                    <Notification color={Color::Danger} light=true onclick={ctx.link().callback(|_| Msg::Remove)}>
                        { format!("{}", error.clone()) }
                    </Notification>
                </div>
            },
        };

        html! {
            <main>
                <BrowserRouter>
                    { notification }
                    <ContextProvider<Auth> context={self.auth.clone()}>
                        { inner }
                    </ContextProvider<Auth>>
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
