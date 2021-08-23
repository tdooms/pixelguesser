use yew::prelude::*;
use yew::utils::NeqAssign;

use super::{Initialize, Master, Navigate, Rating};
use crate::graphql::{Quiz, Round};
use pbs::prelude::*;
use shared::{Player, Session, Stage, Status};

pub enum Msg {
    UpdateStage(Stage),
    NewPlayer(String),
    Guessed(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<Session>,

    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

pub struct InnerManage {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for InnerManage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let changed = match msg {
            Msg::UpdateStage(stage) => self.props.session.stage.neq_assign(stage),
            Msg::NewPlayer(name) => {
                match self.props.session.players.iter().find(|player| player.name == name) {
                    None => {
                        self.props.session.players.push(Player { name, score: 0 });
                        true
                    }
                    Some(_) => false, // TODO: give error
                }
            }
            Msg::Guessed(name) => {
                match self.props.session.players.iter_mut().find(|player| player.name == name) {
                    None => {} // TODO: give error
                    Some(player) => player.score += 1,
                }
                true
            }
        };

        if changed {
            self.props.onchange.emit(self.props.session.clone())
        };
        changed
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let body = match self.props.session.stage {
            Stage::Initial => {
                let onchange = self.link.callback(|name| Msg::NewPlayer(name));
                html! { <Initialize onchange={onchange}/> }
            }
            Stage::Round { round, status: Status::Playing { .. } } => {
                let onguess = self.link.callback(|guess| Msg::Guessed(guess));
                html! { <Master players={self.props.session.players.clone()} onguess={onguess}/> }
            }
            Stage::Round { .. } => html! { <cbs::TitleHero title="revealing" subtitle=""/> }, // TODO: don't show when revealed
            Stage::Ranking { .. } => html! { <cbs::TitleHero title="showing scores" subtitle=""/> },
            Stage::Finished => html! { <Rating quiz={self.props.quiz.clone()} />},
        };

        let stage = self.props.session.stage.clone();
        let rounds = self.props.rounds.len();
        let onchange = self.link.callback(|stage| Msg::UpdateStage(stage));

        html! {
            <Section>
                <Container>
                    { body }
                    <Navigate stage={stage} rounds={rounds} onchange={onchange}/>
                </Container>
            </Section>
        }
    }

    fn destroy(&mut self) {
        // let post = Post::LeaveSession { session_id: self.props.session_id };
        // self.ws_agent.send(Request::Post(post));
        // TODO: stop session
    }
}
