use std::str::FromStr;

use cobul::{Color, Loader};
use yew::*;
use yew_router::prelude::*;

use api::Code;
use components::Toasts;
use create::Create;
use profile::Profile;
use shared::{
    use_auth_manager, use_toast_manager, Route, UseAuthManagerHandle, UseToastManagerHandle,
};

use crate::initializer::Initializer;
use crate::lab::Test;
use crate::library::Library;
use crate::overview::Overview;

mod initializer;
mod lab;
mod library;
mod navbar;
mod overview;
mod search;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
pub fn app() -> Html {
    log::debug!("render main");

    let manager = use_toast_manager();
    let auth = use_auth_manager();

    if auth.loading() {
        return html! { <Loader color={Color::Info} />};
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
    match route {
        Route::Host { quiz_id } => {
            html! { <Initializer quiz_id={*quiz_id}/> }
        }
        Route::Manage { code } => {
            let Code { session_id, quiz_id } = Code::from_str(&code).unwrap();
            html! { <Initializer {quiz_id} {session_id} /> }
        }
        Route::Create => {
            html! { <Create /> }
        }
        Route::Update { quiz_id } => {
            html! { <Create quiz_id={*quiz_id} /> }
        }
        Route::Profile => {
            html! { <Profile /> }
        }
        Route::Library => {
            html! { <Library /> }
        }
        Route::Test => {
            html! { <Test /> }
        }
        Route::Overview | Route::NotFound => {
            html! { <Overview /> }
        }
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    Renderer::<App>::new().render();
}
