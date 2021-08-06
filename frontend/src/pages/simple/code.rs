use yew::prelude::*;

use api::{Fetch, Get, Request, Response};
use pbs::Color;

use crate::agents::WebSocketAgent;
use crate::route::Route;
use crate::utils::string_to_code;

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
        let oninput = self.link.callback(Msg::Input);
        let onjoin = self.link.callback(|_| Msg::Join);
        let oncancel = self.link.callback(|_| Msg::Cancel);

        let field = match self.state {
            State::Available => html! {
                <cbs::SimpleField label="Session code" help="This room is available." help_color={Color::Success} icon_right="fas fa-check">
                    <pbs::Input color={Color::Success} oninput={oninput} />
                </cbs::SimpleField>
            },
            State::Invalid | State::Incorrect => html! {
                <cbs::SimpleField label="Session code" help="The room code is invalid." help_color={Color::Danger} icon_right="fas fa-exclamation-triangle">
                    <pbs::Input color={Color::Danger} oninput={oninput} />
                </cbs::SimpleField>
            },
            _ => html! {
                <cbs::SimpleField label="Session code">
                    <pbs::Input oninput={oninput} />
                </cbs::SimpleField>
            },
        };

        html! {
            <pbs::Section>
                <pbs::Container>
                    { field }
                    <pbs::Buttons>
                        <cbs::IconButton text="Join" color={Color::Link} onclick={onjoin}/>
                        <cbs::IconButton text="Cancel" color={Color::Link} light=true onclick={oncancel}/>
                    </pbs::Buttons>
                </pbs::Container>
            </pbs::Section>
        }
    }
}
