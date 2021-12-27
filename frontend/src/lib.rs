use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Loader;
use crate::pages::*;
use crate::route::Route;
use crate::utils::string_to_code;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod agents;
mod components;
mod constants;
mod error;
mod graphql;
mod pages;
mod route;
mod structs;
mod utils;

pub struct Model;

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <main>
                <BrowserRouter>
                    // <Alerts<Rc<Info>> entries={self.infos.clone()} />
                    <Switch<Route> render={Switch::render(switch)} />
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
