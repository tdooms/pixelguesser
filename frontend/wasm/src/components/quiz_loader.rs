use futures::FutureExt;
use js_sys::Function;
use web_sys::window;
use yew::prelude::*;
use yew::utils::NeqAssign;

use shared::{Request, Response, Session, SessionDiff};

use crate::constants::SESSION_ENDPOINT;
use crate::error::Error;
use crate::graphql::{Quiz, quiz, Round};
use crate::pages::{Host, Manage};
use crate::utils::yew::WebsocketTask;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Kind {
    Host { quiz_id: u64 },
    Manage { session_id: u64 },
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub kind: Kind,
}

pub enum Msg {
    Changed(SessionDiff),
    QuizLoaded(Result<(Quiz, Vec<Round>), Error>),
    WsResponse(Response),
}

pub struct QuizLoader {
    props: Props,
    link: ComponentLink<Self>,

    ws: WebsocketTask,

    session: Option<Session>,
    data: Option<(Quiz, Vec<Round>)>,
}

impl Component for QuizLoader {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url = format!("ws://{}/ws", SESSION_ENDPOINT);
        let mut ws = WebsocketTask::create(url, link.callback(Msg::WsResponse));


        match props.kind {
            Kind::Host { quiz_id } => {
                link.send_future(quiz(quiz_id).map(Msg::QuizLoaded));
                ws.send(&Request::Create { quiz_id })
            }
            Kind::Manage { session_id } => ws.send(&Request::Manage { session_id })
        }

        Self { props, link, ws, session: None, data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg, self.props.kind) {
            (Msg::WsResponse(Response::Created(session)), Kind::Host { .. }) => {
                self.ws.send(&Request::Host { session_id: session.session_id });
                self.session = Some(session);

                if let Some(window) = window() {
                    window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
                }
                true
            }
            (Msg::WsResponse(Response::Updated(session)), _) => {
                if let None = self.data {
                    self.link.send_future(quiz(session.quiz_id).map(Msg::QuizLoaded));
                }
                self.session = Some(session);
                true
            }
            (Msg::WsResponse(_), _) => {
                log::error!("unexpected response");
                false
            }
            (Msg::QuizLoaded(Ok(pair)), _) => {
                self.data = Some(pair);
                true
            }
            (Msg::QuizLoaded(Err(err)), _) => {
                log::error!("{:?}", err);
                false
            }
            (Msg::Changed(SessionDiff { players: None, stage: None }), _) => {
                false
            }
            (Msg::Changed(diff), _) => {
                if let Some(Session { session_id, .. }) = self.session {
                    self.ws.send(&Request::Update { session_id, diff });
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let onchange = self.link.callback(Msg::Changed);

        match (&self.session, &self.data, self.props.kind) {
            (Some(session), Some((quiz, rounds)), Kind::Host { .. }) => {
                html! {<Host session={session.clone()} quiz={quiz.clone()} rounds={rounds.clone()} onchange={onchange} />}
            }
            (Some(session), Some((quiz, rounds)), Kind::Manage { .. }) => {
                html! {<Manage session={session.clone()} quiz={quiz.clone()} rounds={rounds.clone()} onchange={onchange} />}
            }
            _ => html! {},
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }
    }
}
