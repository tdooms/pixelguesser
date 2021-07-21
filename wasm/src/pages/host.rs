use crate::agents::WebSocketAgent;
use crate::components::{lobby, Finish, Pixelate, Scores};
use api::{Alert, Player, Post, Quiz, Reply, Request, Response, Round, Stage, SessionData};
use js_sys::Function;
use std::collections::HashMap;
use yew::prelude::*;
use yew::web_sys::window;

#[derive(Clone, Properties)]
pub struct Props {
    pub session_id: u64,
    pub session: SessionData
}

pub enum Msg {
    Response(Response),
    Revealed,
}

pub struct Host {
    link: ComponentLink<Self>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    props: Props,
    has_manager: bool,
}

impl Component for Host {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| Msg::Response(x)));

        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }

        let quiz_id = props.quiz_id;
        ws_agent.send(Request::Post(Post::StartSession { quiz_id }));

        Self {
            link,
            ws_agent,
            props,
            has_manager: false,
        }
    }

    // TODO: check session validity
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(response) => self.handle_response(response),
            Msg::Revealed => {
                let stage = Stage::Round {round: }
                self.ws_agent.send()
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match (&self.stage, &self.data) {
            (Stage::Initial, _) => lobby(self.data.as_ref(), self.has_manager, &self.players),
            (Stage::Round { round, status }, Some((_, _, rounds))) => html! {
                <Pixelate on_revealed=self.link.callback(|_| Msg::Revealed) status=*status url=rounds[*round].image_url.clone()/>
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

impl Host {
    fn handle_response(&mut self, response: Response) -> bool {
        match response {
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
}
