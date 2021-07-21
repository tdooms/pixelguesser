use crate::agents::WebSocketAgent;
use crate::route::Route;
use api::{Post, Request, Stage, Status};
use pbs::{Color, Size};
use yew::agent::Dispatcher;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub session_id: u64,
    pub stage: Stage,
    pub rounds: usize,
}

pub struct Navigate {
    ws_agent: Dispatcher<WebSocketAgent>,
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Start,
    Pause,
    Resume,
    Next,
    Scores,
    Reveal,
    Finish,
    Leave,
}

impl Component for Navigate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            ws_agent: WebSocketAgent::dispatcher(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let stage = match self.props.stage {
            Stage::Initial => match msg {
                Msg::Start => Stage::Round {
                    round: 0,
                    status: Status::Playing,
                },
                _ => return false,
            },
            Stage::Round { round, status } => match (status, msg) {
                (Status::Playing, Msg::Pause) => Stage::Round {
                    round,
                    status: Status::Paused,
                },
                (Status::Paused, Msg::Resume) => Stage::Round {
                    round,
                    status: Status::Playing,
                },
                (Status::Paused | Status::Playing, Msg::Reveal) => Stage::Round {
                    round,
                    status: Status::Revealing,
                },
                (Status::Revealed, Msg::Next) if round + 1 < self.props.rounds => Stage::Round {
                    round: round + 1,
                    status: Status::Playing,
                },
                (Status::Revealed, Msg::Next) => Stage::Finish,
                (Status::Revealed, Msg::Finish) => Stage::Finish,
                (Status::Revealed, Msg::Scores) => Stage::Round {
                    round,
                    status: Status::Scores,
                },
                (Status::Scores, Msg::Next) => Stage::Round {
                    round: round + 1,
                    status: Status::Playing,
                },
                (Status::Scores, Msg::Finish) => Stage::Finish,
                _ => return false,
            },
            Stage::Finish => {
                return match msg {
                    Msg::Leave => {
                        yew_router::push_route(Route::Overview);
                        false
                    }
                    _ => false,
                }
            }
        };

        let post = Post::ChangeStage {
            session_id: self.props.session_id,
            stage,
        };
        self.ws_agent.send(Request::Post(post));
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // This is terrible
        let show = match self.props.stage {
            Stage::Initial => vec![true, false, false, false, false, false, false, false],
            Stage::Round { round, status } => match status {
                Status::Playing => vec![false, true, false, false, false, true, false, false],
                Status::Paused => vec![false, false, true, false, false, true, false, false],
                Status::Revealing => vec![false; 8],
                Status::Revealed if round + 1 < self.props.rounds => {
                    vec![false, false, false, true, true, false, false, false]
                }
                Status::Revealed => vec![false, false, false, false, false, false, true, false],
                Status::Scores => vec![false, false, false, true, false, false, false, false],
            },
            Stage::Finish => vec![false, false, false, false, false, false, false, true],
        };

        html! {
            <pbs::Buttons centered=true extra="mt-4">
                <pbs::Button text="start" icon="fas fa-play" hidden=!show[0] size=Size::Large color=Color::Primary onclick=self.link.callback(|_| Msg::Start)/>
                <pbs::Button text="pause" icon="fas fa-pause" hidden=!show[1] size=Size::Large color=Color::Light onclick=self.link.callback(|_| Msg::Pause)/>
                <pbs::Button text="resume" icon="fas fa-play" hidden=!show[2] size=Size::Large color=Color::Light onclick=self.link.callback(|_| Msg::Resume)/>
                <pbs::Button text="next" icon="fas fa-forward" hidden=!show[3] size=Size::Large light=true color=Color::Success onclick=self.link.callback(|_| Msg::Next)/>
                <pbs::Button text="scores" icon="fas fa-list-ol" hidden=!show[4] size=Size::Large color=Color::Link outlined=true onclick=self.link.callback(|_| Msg::Scores)/>
                <pbs::Button text="reveal" icon="fas fa-eye" hidden=!show[5] size=Size::Large color=Color::Danger onclick=self.link.callback(|_| Msg::Reveal)/>
                <pbs::Button text="finish" icon="fas fa-flag-checkered" hidden=!show[6] size=Size::Large light=true color=Color::Success onclick=self.link.callback(|_| Msg::Finish)/>
                <pbs::Button text="leave" icon="fas fa-sign-out-alt" hidden=!show[7] size=Size::Large light=true color=Color::Danger onclick=self.link.callback(|_| Msg::Leave)/>
            </pbs::Buttons>
        }
    }
}
