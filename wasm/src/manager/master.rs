use crate::agents::WebSocketAgent;
use api::*;
use pbs::{HeroSize, Size};
use yew::agent::Dispatcher;
use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub session_id: u64,
    pub round: usize,
    pub data: SessionData,
}

pub struct Master {
    ws_agent: Dispatcher<WebSocketAgent>,
    link: ComponentLink<Self>,
    props: Props,
    answer_given: bool,
}

impl Component for Master {
    type Message = u64;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws_agent: WebSocketAgent::dispatcher(),
            link,
            props,
            answer_given: false,
        }
    }

    fn update(&mut self, player_id: Self::Message) -> bool {
        let stage = Stage::Round {
            round: self.props.round,
            status: Status::Revealing,
        };
        let post = Post::ChangeStage {
            session_id: self.props.session_id,
            stage,
        };
        self.ws_agent.send(Request::Post(post));

        let change = ScoreChange {
            player_id,
            change: self.props.data.rounds[self.props.round].points,
            reason: "".to_string(),
        };
        let post = Post::ChangeScores {
            session_id: self.props.session_id,
            diff: vec![change],
        };
        self.ws_agent.send(Request::Post(post));

        self.answer_given = true;
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let view_player = |id: u64, player: &Player| {
            html! {
                <pbs::Button
                    outlined=true size=Size::Large fullwidth=true
                    onclick=self.link.callback(move |_| id) text=player.name.clone()
                />
            }
        };

        let players = self
            .props
            .data
            .players
            .iter()
            .map(|(id, player)| view_player(*id, player));

        let body = html! {
            <pbs::Container extra="has-text-centered">
                <pbs::Title> {&self.props.data.rounds[self.props.round].answer} </pbs::Title>
                <pbs::Subtitle> {self.props.data.rounds[self.props.round].points} {" points"} </pbs::Subtitle>
            </pbs::Container>
        };

        html! {
            <>
                <pbs::Hero size=HeroSize::Medium body=body/>
                { for players }
            </>
        }
    }
}
