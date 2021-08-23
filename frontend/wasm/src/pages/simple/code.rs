use gloo::timers::callback::Timeout;
use pbs::prelude::*;
use pbs::properties::Color;
use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::route::Route;
use crate::utils::misc::string_to_code;
use crate::utils::yew::WebsocketTask;
use crate::constants::SESSION_ENDPOINT;
use shared::{Request, Response};

#[derive(Debug, Clone, PartialEq)]
enum State {
    Available,
    Invalid,
    Incorrect,
    None,
}

pub enum Msg {
    WsResponse(Response),
    Check(Option<u64>),
    Input(String),
    Timer,
    Cancel,
    Join,
}

pub struct Code {
    link: ComponentLink<Self>,
    ws: WebsocketTask,
    timer: Option<Timeout>,

    current: Option<u64>,
    state: State,
}

impl Component for Code {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url = format!("ws://{}/ws", SESSION_ENDPOINT);
        let ws = WebsocketTask::create(url, link.callback(Msg::WsResponse));

        Self { link, ws, timer: None, current: None, state: State::None }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match (msg, self.current) {
            (Msg::Check(Some(id)), Some(current)) if id == current => {
                self.state.neq_assign(State::Available)
            }
            (Msg::Check(None), _) => self.state.neq_assign(State::Invalid),
            (Msg::Input(string), _) => {
                let cloned = self.link.clone();
                self.timer = Some(Timeout::new(200, move || cloned.send_message(Msg::Timer)));

                let res1 = self.state.neq_assign(State::Invalid);
                let res2 = self.current.neq_assign(string_to_code(&string));

                res1 | res2
            }
            (Msg::Timer, Some(session_id)) => {
                self.ws.send(&Request::Read {session_id});
                false
            }
            (Msg::Join, Some(session_id)) if self.state == State::Available => {
                yew_router::push_route(Route::Manage { session_id });
                false
            }
            (Msg::WsResponse(Response::Read(session)), _) => {
                self.state = if session.has_manager { State::Invalid } else { State::Available };
                true
            }
            (Msg::Cancel, _) => {
                yew_router::push_route(Route::Overview);
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(Msg::Input);
        let onjoin = self.link.callback(|_| Msg::Join);
        let oncancel = self.link.callback(|_| Msg::Cancel);

        let (help, help_color, icon_right, input_color) = match self.state {
            State::Available => {
                (Some("This room is available."), Some(Color::Success), Some("fas fa-check"), Some(Color::Success))
            }
            State::Invalid | State::Incorrect => {
                (Some("This room is unavailable."), Some(Color::Danger), Some("fas fa-exclamation-triangle"), Some(Color::Danger))
            }
            State::None => {
                (None, None, None, None)
            }
        };

        html! {
            <Section>
                <Container>
                    <cbs::SimpleField label="Session code" help={help} help_color={help_color} icon_right={icon_right}>
                        <Input color={input_color} oninput={oninput} />
                    </cbs::SimpleField>
                    <Buttons>
                        <Button color={Color::Link} onclick={onjoin} disabled={self.state != State::Available}> {"Join"} </Button>
                        <Button color={Color::Link} light=true onclick={oncancel}> {"Cancel"} </Button>
                    </Buttons>
                </Container>
            </Section>
        }
    }
}
