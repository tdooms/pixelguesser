use std::rc::Rc;
use std::str::FromStr;

use cobul::props::Color;
use cobul::{Loading, Notification};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use yew_router::prelude::*;

use api::{Code, User, AUTH0_CLIENT_ID, AUTH0_DOMAIN};
use create::Create;
use shared::{Auth, Error, Route};

use crate::loader::Loader;
use crate::overview::Overview;

mod loader;
mod overview;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Model {
    auth: Auth,
    error: Option<Rc<Error>>,
}

#[derive(Debug)]
pub enum Msg {
    AddError(Rc<Error>),
    RemoveError,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self { auth: Auth::default(), error: None }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddError(error) => self.error = Some(error),
            Msg::RemoveError => self.error = None,
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
                    <ContextProvider<Errors> context={ctx.link().callback(Msg::AddError)}
                    <ContextProvider<Auth> context={self.auth.clone()}>
                        { inner }
                    </ContextProvider<Auth>>
                    </ContextProvider<Errors>>
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
            let Code { session_id, quiz_id } = Code::from_str(code).unwrap();
            html! { <Loader {quiz_id} {session_id}/> }
        }
        Route::Create => html! { <Create/> },
        Route::Update { quiz_id } => html! { <Create id={*quiz_id}/> },
        Route::Test => html! { <> {"test"} </> },
        Route::Overview => html! { <Overview/> },
        Route::NotFound => html! { <Overview/> },
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
