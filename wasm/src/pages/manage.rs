use crate::agents::WebSocketAgent;
use crate::components::{Initialise, Master, Navigate};
use crate::pages::view_or_loading;
use crate::route::Route;

use api::*;
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub session_id: u64,
}

pub struct Manage {
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,

    data: Option<SessionData>,
    session_id: u64,
    session_closed: bool,
}

impl Component for Manage {
    type Message = Response;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|x| x);
        let mut ws_agent = WebSocketAgent::bridge(callback);

        let session_id = props.session_id;
        let request = Request::Post(Post::JoinSession { session_id });
        ws_agent.send(request);

        Self {
            data: None,
            session_id: props.session_id,
            ws_agent,
            session_closed: false,
        }
    }

    fn update(&mut self, response: Self::Message) -> bool {
        match (response, &mut self.data) {
            (Response::Reply(_, Reply::SessionJoined(data)), _) => {
                self.data = Some(data);
                true
            }
            (Response::Reply(_, Reply::SessionManaged), _) => {
                yew_router::push_route(Route::Code);
                false
            }
            (Response::Alert(_, Alert::StageChanged(stage)), Some(data)) => {
                data.stage = stage;
                true
            }
            (Response::Alert(_, Alert::PlayerAdded(id, name)), Some(data)) => {
                data.players.insert(id, Player { name, score: 0 });
                true
            }
            (Response::Error(Error::SessionDoesNotExist(_)), _)
            | (Response::Alert(_, Alert::SessionStopped), _) => {
                yew_router::push_route(Route::Overview);
                self.session_closed = true;
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.session_id = props.session_id;
        true
    }

    fn view(&self) -> Html {
        let view_stage = |data: &SessionData| {
            let html = match data.stage {
                Stage::Initial => html! {
                    <Initialise session_id=self.session_id/>
                },
                Stage::Round {
                    round,
                    status: Status::Playing | Status::Paused,
                } => html! {
                    <Master session_id=self.session_id round=round data=data.clone()/>
                },
                Stage::Scores { .. } | Stage::Finish | Stage::Round { .. } => html! {},
            };
            html! {
                <>
                    { html }
                    <Navigate session_id=self.session_id stage=data.stage.clone() rounds=data.rounds.len()/>
                </>
            }
        };

        html! {
            <section class="section">
                <div class="container">
                    { view_or_loading(self.data.as_ref(), view_stage) }
                </div>
            </section>
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
