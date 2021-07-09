use crate::agents::WebSocketAgent;
use crate::route::Route;
use crate::utils::string_to_code;

use api::{Fetch, Get, Request, Response};
use yew::prelude::*;

enum State {
    Available,
    Invalid,
    Incorrect,
    None,
}

pub enum Msg {
    Response(Response),
    Input(String),
    Cancel,
    Join,
}

pub struct Code {
    link: ComponentLink<Self>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,

    current: String,
    state: State,
}

impl Component for Code {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws_agent: WebSocketAgent::bridge(link.callback(Msg::Response)),
            link,
            current: String::new(),
            state: State::None,
        }
    }

    // FIXME: this code is too verbose
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Response::Fetch(Fetch::SessionInvalid(session_id))) => {
                let res = string_to_code(&self.current) == Some(session_id);
                if res {
                    self.state = State::Invalid
                }
                res
            }
            Msg::Response(Response::Fetch(Fetch::SessionAvailable(session_id))) => {
                let res = string_to_code(&self.current) == Some(session_id);
                if res {
                    self.state = State::Available
                }
                res
            }
            Msg::Input(string) => {
                self.current = string;

                self.state = match string_to_code(&self.current) {
                    Some(session_id) => {
                        let get = Get::CheckSession { session_id };
                        self.ws_agent.send(Request::Get(get));
                        State::None
                    }
                    None => State::Incorrect,
                };

                true
            }
            Msg::Join => {
                if let Some(session_id) = string_to_code(&self.current) {
                    yew_router::push_route(Route::Manage { session_id });
                }
                false
            }
            Msg::Cancel => {
                yew_router::push_route(Route::Overview);
                false
            }
            Msg::Response(_) => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(|data: InputData| Msg::Input(data.value));
        let onjoin = self.link.callback(|_| Msg::Join);
        let oncancel = self.link.callback(|_| Msg::Cancel);

        // TODO: dedicated field components
        let (control, color, icon, error) = match self.state {
            State::Available => (
                "has-icons-right",
                "is-success",
                "fas fa-check",
                "This room is available",
            ),
            State::Invalid | State::Incorrect => (
                "has-icons-right",
                "is-danger",
                "fas fa-exclamation-triangle",
                "This room is code is invalid",
            ),
            State::None => ("", "", "", ""),
        };

        html! {
            <section class="section">
                <div class="container">
                    <div class="field">
                        <label class="label">{"Session code"}</label>
                        <div class=classes!("control", control)>
                            <input class=classes!("input", "is-large", color) type="text" oninput=oninput/>
                            <span class="icon is-small is-right">
                                <i class={icon}></i>
                            </span>
                        </div>
                    <p class=classes!("help", color)>{error}</p>
                    </div>

                    <div class="field is-grouped">
                        <div class="control">
                            <button class="button is-link is-large" onclick=onjoin>{"Join"}</button>
                        </div>
                        <div class="control">
                            <button class="button is-link is-light is-large" onclick=oncancel>{"Cancel"}</button>
                        </div>
                    </div>

                </div>
            </section>
        }
    }
}
