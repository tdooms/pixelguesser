use std::str::FromStr;

use cobul::{Color, Loader};
use tracing_wasm::{WASMLayerConfig, WASMLayerConfigBuilder};
use yew::*;
use yew_router::prelude::*;

use admin::{Admin, Sessions};
use api::Code;
use components::Toasts;
use create::Create;
use profile::Profile;
use shared::{
    use_auth_manager, use_toast_manager, Route, UseAuthManagerHandle, UseToastManagerHandle,
};

use crate::initializer::Initializer;
use crate::test::Test;
use crate::library::Library;
use crate::overview::Overview;

mod configuration;
mod initializer;
mod library;
mod navbar;
mod overview;
mod search;
mod test;

#[function_component(App)]
pub fn app() -> Html {
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
                    <Switch<Route> render={switch} />
                </ContextProvider<UseToastManagerHandle>>
                </ContextProvider<UseAuthManagerHandle>>
            </BrowserRouter>
        </main>
    }
}

fn switch(routes: Route) -> Html {
    let fallback = html! { <Loader /> };
    match routes {
        Route::Host { quiz_id } => {
            html! { <Suspense {fallback}> <Initializer {quiz_id} /> </Suspense> }
        }
        Route::Manage { code } => {
            let Code { session_id, quiz_id } = Code::from_str(&code).unwrap();
            tracing::warn!("{} {}", session_id, quiz_id);
            html! { <Suspense {fallback}> <Initializer {quiz_id} {session_id} /> </Suspense> }
        }
        Route::Create => {
            html! { <Create /> }
        }
        Route::Update { quiz_id } => {
            html! { <Create {quiz_id} /> }
        }
        Route::Admin => {
            html! { <Admin /> }
        }
        Route::Sessions => {
            html! { <Suspense {fallback}> <Sessions /> </Suspense> }
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
    let config = WASMLayerConfigBuilder::default().set_max_level(tracing::Level::DEBUG).build();

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default_with_config(config);

    Renderer::<App>::new().render();
}
