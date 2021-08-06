use api::*;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::WebSocketAgent;
use crate::pages::manage::{Initialize, Master, Navigate, Rating};
use crate::route::Route;

pub enum Msg {
    ChangeStage(Stage),
    PlayerAdded(String),
    Guessed(u64),
    Response(Response),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session_id: u64,
    pub session: Session,
}

pub struct InnerManage {
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for InnerManage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { ws_agent: WebSocketAgent::bridge(link.callback(|x| Msg::Response(x))), link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(Response::Reply(_, Reply::SessionManaged)) => {
                // TODO: what does this do?
                yew_router::push_route(Route::Code);
                false
            }
            Msg::Response(Response::Alert(_, Alert::StageChanged(stage))) => {
                self.props.session.stage = stage;
                true
            }
            Msg::Response(Response::Alert(_, Alert::PlayerAdded(id, name))) => {
                self.props.session.players.insert(id, Player { name, score: 0 });
                true
            }
            Msg::Response(Response::Alert(_, Alert::SessionStopped)) => {
                yew_router::push_route(Route::Overview);
                // TODO: give warning
                false
            }
            Msg::Response(Response::Error(Error::SessionDoesNotExist(_))) => {
                yew_router::push_route(Route::Overview);
                // TODO: give error
                false
            }
            Msg::Response(_) => false,
            Msg::Guessed(player_id) => {
                let rounds = self.props.session.rounds.len();
                let session_id = self.props.session_id;

                match self.props.session.stage.perform(Action::Reveal, rounds) {
                    Some(stage) => {
                        let post = Post::ChangeStage { session_id, stage };
                        self.ws_agent.send(Request::Post(post))
                    }
                    None => {} // TODO: error
                }

                if let Stage::Round { round, .. } = self.props.session.stage {
                    let change = ScoreChange {
                        player_id,
                        change: self.props.session.rounds[round].points,
                        reason: "".to_string(),
                    };

                    let post = Post::ChangeScores { session_id, diff: vec![change] };
                    self.ws_agent.send(Request::Post(post));
                } else {
                    // Error
                }

                false
            }
            Msg::PlayerAdded(name) => {
                let post = Post::AddPlayer { session_id: self.props.session_id, name };
                self.ws_agent.send(Request::Post(post));
                false
            }
            Msg::ChangeStage(stage) => {
                let post = Post::ChangeStage { session_id: self.props.session_id, stage };
                self.ws_agent.send(Request::Post(post));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let body = match self.props.session.stage {
            Stage::Initial => {
                let onchange = self.link.callback(|name| Msg::PlayerAdded(name));
                html! { <Initialize onchange={onchange}/> }
            }
            Stage::Round { round, status: Status::Playing { .. } } => {
                let onclick = self.link.callback(|guess| Msg::Guessed(guess));
                html! { <Master players={self.props.session.players.clone()} onclick={onclick}/> }
            }
            Stage::Round { .. } => html! { <cbs::TitleHero title="revealing" subtitle=""/> }, // TODO: don't show when revealed
            Stage::Scores { .. } => html! { <cbs::TitleHero title="showing scores" subtitle=""/> },
            Stage::Finish => html! { <Rating quiz={self.props.session.quiz.clone()} />},
        };

        let stage = self.props.session.stage.clone();
        let rounds = self.props.session.rounds.len();
        let onchange = self.link.callback(|stage| Msg::ChangeStage(stage));

        html! {
            <pbs::Section>
                <pbs::Container>
                    { body }
                    <Navigate stage={stage} rounds={rounds} onchange={onchange}/>
                </pbs::Container>
            </pbs::Section>
        }
    }

    fn destroy(&mut self) {
        let post = Post::LeaveSession { session_id: self.props.session_id };
        self.ws_agent.send(Request::Post(post));
    }
}
