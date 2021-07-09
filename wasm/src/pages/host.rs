use crate::agents::WebSocketAgent;
use crate::components::{lobby, Finish, Pixelate, Scores};
use api::{Alert, Player, Post, Quiz, Reply, Request, Response, Round, Stage};
use js_sys::Function;
use std::collections::HashMap;
use yew::prelude::*;
use yew::web_sys::window;

#[derive(Clone, Properties)]
pub struct Props {
    pub quiz_id: i64,
}

pub struct Host {
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    props: Props,

    data: Option<(u64, Quiz, Vec<Round>)>,
    players: HashMap<u64, Player>,
    stage: Stage,

    has_manager: bool,
}

impl Component for Host {
    type Message = Response;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| x));

        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }

        let quiz_id = props.quiz_id;
        ws_agent.send(Request::Post(Post::StartSession { quiz_id }));

        Self {
            ws_agent,
            stage: Stage::Initial,
            props,
            data: None,
            has_manager: false,
            players: HashMap::new(),
        }
    }

    // TODO: check session validity
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Response::Reply(session_id, Reply::SessionCreated(quiz, rounds)) => {
                self.data = Some((session_id, quiz, rounds));
                true
            }
            Response::Alert(_, Alert::PlayerAdded(id, name)) => {
                self.players.insert(id, Player { score: 0, name });
                true
            }
            Response::Alert(_, Alert::ScoreChanged(diff)) => {
                for change in diff {
                    match self.players.get_mut(&change.player_id) {
                        Some(player) => player.score += change.change,
                        None => log::debug!("no player with given id"),
                    }
                }
                true
            }
            Response::Alert(_, Alert::ManagerJoined) => {
                self.has_manager = true;
                true
            }
            Response::Alert(_, Alert::ManagerLeft) => {
                // TODO: alert
                self.has_manager = false;
                true
            }
            Response::Alert(_, Alert::StageChanged(stage)) => {
                self.stage = stage;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match (&self.stage, &self.data) {
            (Stage::Initial, _) => lobby(self.data.as_ref(), self.has_manager, &self.players),
            (Stage::Round { round, status }, Some((session_id, _, rounds))) => html! {
                <Pixelate round=*round session_id=*session_id status=*status url=rounds[*round].image_url.clone()/>
            },
            (Stage::Scores { round: _ }, Some((session_id, _, _))) => html! {
                <section class="section">
                    <div class="container">
                        <Scores session_id=*session_id players=self.players.clone()/>
                    </div>
                </section>
            },
            (Stage::Finish, Some((session_id, quiz, _))) => {
                html! {<Finish session_id=*session_id players=self.players.clone() quiz=quiz.clone()/>}
            }
            _ => html! {<p> {"handul eror"} </p>},
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }

        if let Some((session_id, _, _)) = self.data {
            let post = Post::StopSession { session_id };
            self.ws_agent.send(Request::Post(post))
        }
    }
}
