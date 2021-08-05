// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod agents;
mod components;
mod create;
mod globals;
mod host;
mod manager;
mod notifications;
mod pages;
mod route;
mod utils;

use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::agents::{AlertAgent, WebSocketAgent};
use crate::notifications::Notification;
use crate::pages::*;
use crate::route::Route;

pub struct Model {
    // Keeps WebSocket connection alive
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    alert_agent: Box<dyn Bridge<AlertAgent>>,

    alerts: Vec<Rc<Notification>>,
    link: ComponentLink<Self>,
}

impl Component for Model {
    type Message = Rc<Notification>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            _ws_agent: WebSocketAgent::bridge(Callback::noop()),
            alert_agent: AlertAgent::bridge(link.callback(|x| x)),
            alerts: Vec::new(),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.alerts.push(msg);
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <cbs::Alerts<Rc<Notification>> entries={self.alerts.clone()} />
                <Router<Route> render={Router::render(switch)} />
            </main>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Host { quiz_id } => html! { <HostLoader quiz_id={*quiz_id}/> },
        Route::Manage { session_id } => html! { <ManageLoader session_id={*session_id}/> },
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
