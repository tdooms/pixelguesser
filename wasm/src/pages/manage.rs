use crate::agents::WebSocketAgent;
use crate::manager::{Initialize, Master, Navigate};
use crate::pages::view_or_loading;
use crate::route::Route;

use api::*;
use yew::prelude::*;
use yewtil::NeqAssign;

pub enum Msg {
    Guessed(Option<u64>),
    Response(Response),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session_id: u64,
    pub session: SessionData,
}

pub struct Manage {
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    props: Props,
    session_closed: bool,
}

impl Component for Manage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws_agent: WebSocketAgent::bridge(link.callback(|x| x)),
            props,
            session_closed: false,
        }
    }

    fn update(&mut self, response: Self::Message) -> bool {
        match response {
            Response::Reply(_, Reply::SessionManaged) => {
                yew_router::push_route(Route::Code);
                false
            }
            Response::Alert(_, Alert::StageChanged(stage)) => {
                self.props.session.stage = stage;
                true
            }
            Response::Alert(_, Alert::PlayerAdded(id, name)) => {
                self.props
                    .session
                    .players
                    .insert(id, Player { name, score: 0 });
                true
            }
            Response::Error(Error::SessionDoesNotExist(_))
            | Response::Alert(_, Alert::SessionStopped) => {
                yew_router::push_route(Route::Overview);
                self.session_closed = true;
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assing(props)
    }

    fn view(&self) -> Html {
        let view_master = match self.props.session.stage {
            Stage::Initial => html! {
                <Initialize session_id=self.props.session_id/>
            },
            Stage::Round {
                round,
                status: Status::Playing | Status::Paused,
            } => html! {
                <Master session_id=self.props.session_id round=round data=data.clone()/>
            },
            Stage::Finish | Stage::Round { .. } => html! {},
        };

        html! {
            // <pbs::Section>
                <pbs::Container>
                    // { view_master }
                    // <Navigate session_id=self.props.session_id stage=data.stage.clone() rounds=data.rounds.len()/>
                </pbs::Container>
            // </pbs:Section>
        }
    }

    fn destroy(&mut self) {
        if !self.session_closed {
            let post = Post::LeaveSession {
                session_id: self.session_id,
            };
            self.ws_agent.send(Request::Post(post));
        }
    }
}
