use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::Alerts;
use crate::components::Kind;
use crate::components::QuizLoader;
use crate::pages::*;
use crate::route::Route;
use crate::structs::{Error, Info};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod agents;
mod components;
mod constants;
pub mod graphql;
mod pages;
mod route;
mod structs;
mod utils;
mod error;

pub enum Msg {
    Error(Rc<Error>),
    Info(Rc<Info>),
}

pub struct Model {
    errors: Vec<Rc<Error>>,
    infos: Vec<Rc<Info>>,

    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { errors: vec![], infos: vec![], link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Error(err) => self.errors.push(err),
            Msg::Info(_) => {}
        }
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Alerts<Rc<Info>> entries={self.infos.clone()} />
                <Router<Route> render={Router::render(switch)} />
            </main>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Host { quiz_id } => html! { <QuizLoader kind={Kind::Host{ quiz_id: *quiz_id }}/> },
        Route::Manage { session_id } => html! { <QuizLoader kind={Kind::Manage{ session_id: *session_id }}/> },
        Route::Code => html! { <Code/> },
        Route::Create => html! { <Create/> },
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
