use std::rc::Rc;
use std::str::FromStr;

use cobul::Loader;
use cobul::{Button, Color, ModalCard, Notification};

use yew::*;
use yew_router::prelude::*;

use api::Code;
use create::Create;
use shared::{Auth, Error, Errors, Route};
use ywt::callback;

use crate::initializer::Initializer;
use crate::lab::Test;
use crate::overview::Overview;

mod dropdown;
mod initializer;
mod lab;
mod navbar;
mod overview;
mod profile;
mod search;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn view_error(error: Option<Rc<Error>>, onexit: Callback<()>) -> Html {
    let footer = html! { <Button> <p> {"Leave"} </p> </Button> };

    let error = match error {
        Some(error) => error,
        None => return html! {},
    };

    match &*error {
        Error::Internal(msg) => html! {
            <ModalCard title="Internal error" {footer}>
                <p> {msg.to_string()} </p>
            </ModalCard>
        },
        Error::Api(msg) => html! {
            <ModalCard title="Api error" {footer}>
                <p> {msg.to_string()} </p>
            </ModalCard>
        },
        Error::Warning(msg) => html! {
            <div style="position:absolute; top:55px; left:55px; z-index: 10">
                <Notification color={Color::Danger} light=true ondelete={onexit}>
                { format!("{}", msg.clone()) }
                </Notification>
            </div>
        },
        Error::Info(msg) => html! {
            <div style="position:absolute; top:55px; left:55px; z-index: 10">
                <Notification color={Color::Info} light=true ondelete={onexit}>
                { format!("{}", msg.clone()) }
                </Notification>
            </div>
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let error = use_state(|| None);
    let trigger = use_state(|| ());

    let auth = use_state(|| Auth::new(callback!(trigger; move |_| trigger.set(()))));

    let inner = match &auth.user() {
        Err(true) => html! { <Loader />},
        _ => html! { <Switch<Route> render={Switch::render(switch)} /> },
    };

    let onexit = callback!(error; move |_| error.set(None));
    let onadd = callback!(error; move |e| error.set(Some(Rc::new(e))));

    let notification = view_error((*error).clone(), onexit);

    html! {
        <main>
            <BrowserRouter>
                { notification }
                <ContextProvider<Errors> context={onadd}>
                <ContextProvider<Auth> context={(*auth).clone()}>
                    { inner }
                </ContextProvider<Auth>>
                </ContextProvider<Errors>>
            </BrowserRouter>
        </main>
    }
}

fn switch(route: &Route) -> Html {
    let fallback = html! { <Loader/> };

    match route {
        Route::Host { quiz_id } => {
            html! { <Initializer quiz_id={*quiz_id}/> }
        }
        Route::Manage { code } => {
            let Code { session_id, quiz_id } = Code::from_str(&code).unwrap();
            html! { <Initializer {quiz_id} {session_id}/> }
        }
        Route::Create => {
            html! { <Create/> }
        }
        Route::Update { quiz_id } => {
            html! { <Create quiz_id={*quiz_id}/> }
        }
        Route::Test => {
            html! { <Test/> }
        }
        Route::Overview | Route::NotFound => {
            html! {<Suspense {fallback}><Overview/></Suspense>}
        }
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
