use crate::agents::WebSocketAgent;
use api::*;
use bulma::Button;
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
        let view_player = |(id, player): (&u64, &Player)| {
            let id = *id;
            let onclick = self.link.callback(move |_| id);

            html! {
                <Button outlined=true large=true fullwidth=true onclick=onclick text=player.name.clone() />
            }
        };

        html! {
            <>
            <section class="hero is-medium">
                <div class="hero-body">
                    <div class="container has-text-centered">
                        <p class="title">
                            {&self.props.data.rounds[self.props.round].answer}
                        </p>
                        <p class="subtitle">
                            {self.props.data.rounds[self.props.round].points} {" points"}
                        </p>
                    </div>
                </div>
            </section>

            { for self.props.data.players.iter().map(view_player) }

            </>
        }
    }
}
