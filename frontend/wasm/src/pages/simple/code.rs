use yew::prelude::*;

use api::{Fetch, Get, Request, Response};
use pbs::Color;

use crate::agents::WebSocketAgent;
use crate::route::Route;
use crate::utils::{string_to_code, session_get};
use crate::constants::SESSION_ENDPOINT;
use yewtil::NeqAssign;
use gloo::timers::callback::Timeout;

enum State {
    Available,
    Invalid,
    Incorrect,
    None,
}

pub enum Msg {
    Response(bool),
    Input(String),
    Timer,
    Cancel,
    Join,
}

pub struct Code {
    link: ComponentLink<Self>,
    timer: Option<Timeout>,

    current: Option<u64>,
    state: State,
}

impl Component for Code {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            timer: None,
            current: None,
            state: State::None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match (msg, self.current) {
            (Msg::Response(true), _) => {
                self.state.neq_assign(State::Available)
            }
            (Msg::Response(false), _) => {
                self.state.neq_assign(State::Invalid)
            }
            (Msg::Input(string), _) => {
                self.timer = Some(Timeout::new(200, self.link.callback(Msg::Timer)));
                self.current.neq_assign(string_to_code(&string))
            }
            (Msg::Timer, Some(session_id)) => {
                let future = session_get(format!("/{}/check", session_id), Msg::Response);
                self.link.send_future(future);
                false
            }
            (Msg::Join, Some(session_id)) => {
                yew_router::push_route(Route::Manage { session_id });
                false
            }
            (Msg::Cancel, _) => {
                yew_router::push_route(Route::Overview);
                false
            },
            _ => {}
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
