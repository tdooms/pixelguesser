use instant::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_services::timeout::TimeoutTask;
use yew_services::TimeoutService;

use crate::agents::{AlertAgent, WebSocketAgent};
use crate::components::Notifications;
use crate::notifications::Notification;
use crate::pages::*;
use crate::route::Route;
use api::Alert;
use std::rc::Rc;

mod agents;
mod components;
mod globals;
mod host;
mod manager;
mod notifications;
mod pages;
mod route;
mod utils;

pub struct Model {
    // Keeps WebSocket connection alive
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    alert_agent: Box<dyn Bridge<AlertAgent>>,

    alerts: Vec<Rc<Notification>>,
    link: ComponentLink<Self>,
    timer: TimeoutTask,
}

impl Component for Model {
    type Message = Rc<Notification>;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ws_agent = WebSocketAgent::bridge(link.callback(|_| {}));
        // let timer = TimeoutService::spawn(Duration::from_millis(33), link.callback(|_| ()));

        Self {
            _ws_agent: ws_agent,
            alert_agent: AlertAgent::bridge(link.callback(|x| x)),
            alerts: Vec::new(),
            timer,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        // self.timer = TimeoutService::spawn(Duration::from_millis(33), self.link.callback(|_| ()));
        self.alerts.push(msg);
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <Alerts entries=self.alerts>
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
