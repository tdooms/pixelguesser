use std::collections::HashMap;

use yew::prelude::*;
use yewtil::NeqAssign;

use api::{Alert, Post, Reply, Request, Response, Session, Stage};

use crate::agents::WebSocketAgent;
use crate::pages::host::InnerHost;

pub enum Msg {
    Response(Response),
    Revealed,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HostLoaderProps {
    pub quiz_id: i64,
}

pub struct Host {
    props: HostLoaderProps,
    session: Option<(u64, Session, bool)>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Component for Host {
    type Message = Msg;
    type Properties = HostLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| x));
        ws_agent.send(Request::Post(Post::StartSession { quiz_id: props.quiz_id }));

        Self { props, session: None, ws_agent }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg, &self.session) {
            (Msg::Response(Response::Reply(session_id, reply)), None) => match reply {
                Reply::SessionCreated(quiz, rounds) => {
                    let players = HashMap::new();
                    let session = Session { stage: Stage::Initial, quiz, rounds, players };

                    self.session = Some((session_id, session, false));
                    true
                }
                _ => false,
            },
            (Msg::Response(Response::Alert(session_id, alert)), Some(mut data)) => match alert {
                Alert::SessionStopped => {
                    // TODO: error
                }
                Alert::ManagerChanged(bool) => {
                    // TODO: alert
                    data.2 = bool;
                    true
                }
                Alert::ScoreChanged(diff) => {
                    for change in diff {
                        match data.1.players.get_mut(&change.player_id) {
                            Some(player) => player.score += change.change,
                            None => log::debug!("no player with given id"), // TODO: error?
                        }
                    }
                    true
                }
                Alert::StageChanged(stage) => data.1.stage = stage,
                Alert::PlayerAdded(id, name) => {
                    session.players.insert(id, Player { score: 0, name });
                    true
                }
            },
        }

        match (msg, &self.session) {
            (Response::Reply(session_id, Reply::SessionCreated(quiz, rounds)), None) => {
                let players = HashMap::new();
                let session = Session { stage: Stage::Initial, quiz, rounds, players };

                self.session = Some((session_id, session, false));
                true
            }
            (Response::Alert(_, Alert::PlayerAdded(id, name)), Some((_, mut session, _))) => {
                session.players.insert(id, Player { score: 0, name });
                true
            }
            (Response::Alert(_, Alert::ScoreChanged(diff)), Some((_, mut session, _))) => {
                for change in diff {
                    match session.players.get_mut(&change.player_id) {
                        Some(player) => player.score += change.change,
                        None => log::debug!("no player with given id"), // TODO: error?
                    }
                }
                true
            }
            (Response::Alert(_, Alert::Man), Some((_, _, mut manager))) => {
                // TODO: alert
                manager = true;
                true
            }
            (Response::Alert(_, Alert::ManagerLeft), Some((_, _, mut manager))) => {
                // TODO: alert
                manager = false;
                true
            }
            (Response::Alert(_, Alert::StageChanged(stage)), Some((_, mut session))) => {
                session.stage = stage;
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.session {
            Some((session_id, session)) => {
                html! {<InnerHost session={session.clone()} session_id={*session_id}/>}
            }
            None => html! {},
        }
    }
}
