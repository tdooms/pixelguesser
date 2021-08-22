use gloo::timers::callback::Timeout;
use yew::prelude::*;

use pbs::prelude::*;
use yew::utils::NeqAssign;

use crate::route::Route;
use crate::utils::string_to_code;

#[derive(Debug, Clone, PartialEq)]
enum State {
    Available,
    Invalid,
    Incorrect,
    None,
}

pub enum Msg {
    Check(Option<u64>),
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
        Self { link, timer: None, current: None, state: State::None }
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
                // TODO: send session check
                false
            }
            (Msg::Join, Some(session_id)) if self.state == State::Available => {
                yew_router::push_route(Route::Manage { session_id });
                false
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

        let field = match self.state {
            State::Available => html! {
                <cbs::SimpleField label="Session code" help="This room is available." help_color={Color::Success} icon_right="fas fa-check">
                    <Input color={Color::Success} oninput={oninput} />
                </cbs::SimpleField>
            },
            State::Invalid | State::Incorrect => html! {
                <cbs::SimpleField label="Session code" help="The room code is invalid." help_color={Color::Danger} icon_right="fas fa-exclamation-triangle">
                    <Input color={Color::Danger} oninput={oninput} />
                </cbs::SimpleField>
            },
            _ => html! {
                <cbs::SimpleField label="Session code">
                    <Input oninput={oninput} />
                </cbs::SimpleField>
            },
        };

        html! {
            <Section>
                <Container>
                    { field }
                    <Buttons>
                        <Button color={Color::Link} onclick={onjoin} disabled={self.state != State::Available}> {"Join"} </Button>
                        <Button color={Color::Link} light=true onclick={oncancel}> {"Cancel"} </Button>
                    </Buttons>
                </Container>
            </Section>
        }
    }
}
