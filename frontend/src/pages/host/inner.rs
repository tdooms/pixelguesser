use js_sys::Function;
use yew::prelude::*;
use yew::web_sys::window;

use api::{Alert, Player, Post, Request, Response, Session, Stage, Status};

use crate::agents::WebSocketAgent;
use crate::components::Pixelate;
use crate::pages::host::{Finish, Lobby, Scores};

#[derive(Clone, Properties)]
pub struct Props {
    pub session_id: u64,
    pub session: Session,
}

pub enum Msg {
    Response(Response),
    Revealed,
}

pub struct InnerHost {
    link: ComponentLink<Self>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    props: Props,
    has_manager: bool,
}

impl Component for InnerHost {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }

        let ws_agent = WebSocketAgent::bridge(link.callback(|x| Msg::Response(x)));
        Self { ws_agent, link, props, has_manager: false }
    }

    // TODO: check session validity
    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Response(response) => self.handle_response(response),
            Msg::Revealed => {
                match self.props.session.stage {
                    Stage::Round { round, status: Status::Revealing } => {
                        let stage = Stage::Round { round, status: Status::Revealed };
                        let post = Post::ChangeStage { session_id: self.props.session_id, stage };
                        self.ws_agent.send(Request::Post(post))
                    }
                    _ => {}
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        match &self.props.session.stage {
            Stage::Initial => {
                html! {
                    <Lobby session={self.props.session.clone()}
                        session_id={self.props.session_id}
                        has_manager={self.has_manager}
                    />
                }
            }
            Stage::Round { round, status } => {
                html! {
                    <Pixelate on_revealed={self.link.callback(|_| Msg::Revealed)}
                        status={*status}
                        url={self.props.session.rounds[*round].image_url.clone()}
                    />
                }
            }
            Stage::Scores { round } => {
                html! {
                    <pbs::Section>
                        <pbs::Container>
                            <Scores players={self.props.session.players.clone()}/>
                        </pbs::Container>
                    </pbs::Section>
                }
            }
            Stage::Finish => {
                html! {
                    <Finish session_id={self.props.session_id}
                        players={self.props.session.players.clone()}
                        quiz={self.props.session.quiz.clone()}
                    />
                }
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }

        let session_id = self.props.session_id;
        let post = Post::StopSession { session_id };
        self.ws_agent.send(Request::Post(post))
    }
}

impl InnerHost {
    fn handle_response(&mut self, response: Response) -> bool {
        match response {
            Response::Alert(_, Alert::PlayerAdded(id, name)) => {
                self.props.session.players.insert(id, Player { score: 0, name });
                true
            }
            Response::Alert(_, Alert::ScoreChanged(diff)) => {
                for change in diff {
                    match self.props.session.players.get_mut(&change.player_id) {
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
                self.props.session.stage = stage;
                true
            }
            _ => false,
        }
    }
}
