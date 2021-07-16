use instant::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_services::timeout::TimeoutTask;
use yew_services::TimeoutService;

use crate::agents::WebSocketAgent;
use crate::components::Notifications;
use crate::pages::*;
use crate::route::Route;

mod agents;
mod components;
mod globals;
mod notifications;
mod pages;
mod route;
mod utils;

pub struct Model {
    // Keeps WebSocket connection alive
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    link: ComponentLink<Self>,
    timer: TimeoutTask,
}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ws_agent = WebSocketAgent::bridge(link.callback(|_| {}));
        let timer = TimeoutService::spawn(Duration::from_millis(33), link.callback(|_| ()));
        Self {
            _ws_agent: ws_agent,
            timer,
            link,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.timer = TimeoutService::spawn(Duration::from_millis(33), self.link.callback(|_| ()));
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Notifications/>
                <Router<Route> render=Router::render(switch) />
            </main>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Host { quiz_id } => html! { <Host quiz_id=*quiz_id/> },
        Route::Code => html! { <Code/> },
        Route::Manage { session_id } => html! { <Manage session_id=*session_id/> },
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
