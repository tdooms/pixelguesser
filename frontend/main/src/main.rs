use std::rc::Rc;
use std::str::FromStr;

use cobul::props::Color;
use cobul::{Loading, Notification};
use yew::prelude::*;
use yew_router::prelude::*;

use api::{Code, User};
use create::Create;
use shared::{callback, Auth, Error, Errors, Route};

use crate::loader::Loader;
use crate::overview::Overview;
use crate::test::Test;

pub mod dropdown;
mod loader;
pub mod navbar;
mod overview;
mod profile;
mod test;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
pub fn app() -> Html {
    let error = use_state(|| None);
    let trigger = use_state(|| ());

    let auth = use_state(|| Auth::new(callback!(trigger; move |_| trigger.set(()))));
    log::trace!("main app {:?}, {:?}", error.as_ref(), auth.user().map(|x| x.nickname));

    let inner = match &auth.user() {
        Err(true) => html! { <Loading />},
        _ => html! { <Switch<Route> render={Switch::render(switch)} /> },
    };

    let onremove = callback!(error; move |_| error.set(None));
    let onadd = callback!(error; move |e| error.set(Some(Rc::new(e))));

    let view_error = |error: &Rc<Error>| {
        html! {
            <div style="position:absolute; top:55px; left:55px; z-index: 10">
                <Notification color={Color::Danger} light=true onclick={onremove}>
                    { format!("{}", error.clone()) }
                </Notification>
            </div>
        }
    };

    let notification = error.as_ref().map(view_error).unwrap_or_default();

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
    log::trace!("switch {:?}", route);
    let fallback = html! { <Loading/> };

    match route {
        Route::Host { quiz_id } => {
            html! { <Loader quiz_id={*quiz_id}/> }
        }
        Route::Manage { code } => {
            let Code { session_id, quiz_id } = Code::from_str(&code).unwrap();
            html! { <Loader {quiz_id} {session_id}/> }
        }
        Route::Create => {
            html! {<Suspense {fallback}><Create/></Suspense>}
        }
        Route::Update { quiz_id } => {
            html! { <Suspense {fallback}> <Create id={*quiz_id}/> </Suspense> }
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
