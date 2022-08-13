use std::str::FromStr;

use cobul::Loader;

use yew::*;
use yew_router::prelude::*;

use api::Code;
use auth::{Login, Profile, Signup};
use create::Create;
use shared::{
    use_auth_manager, use_toast_manager, Route, UseAuthManagerHandle, UseToastManagerHandle,
};

use crate::initializer::Initializer;
use crate::lab::Test;
use crate::overview::Overview;
use components::Toasts;

mod dropdown;
mod initializer;
mod lab;
mod navbar;
mod overview;
mod search;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
pub fn app() -> Html {
    let manager = use_toast_manager();
    let auth = use_auth_manager();

    if auth.loading() {
        return html! { <Loader />};
    }

    html! {
        <main>
            <BrowserRouter>
                <ContextProvider<UseAuthManagerHandle> context={auth}>
                <ContextProvider<UseToastManagerHandle> context={manager}>
                    <Toasts />
                    <Switch<Route> render={Switch::render(switch)} />
                </ContextProvider<UseToastManagerHandle>>
                </ContextProvider<UseAuthManagerHandle>>
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
        Route::Login => {
            html! { <Login/> }
        }
        Route::Signup => {
            html! { <Signup/> }
        }
        Route::Profile => {
            html! { <Profile/> }
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
    Renderer::<App>::new().render();
}
