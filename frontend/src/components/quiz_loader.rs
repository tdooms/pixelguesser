use futures::FutureExt;
use js_sys::Function;
use web_sys::window;
use yew::prelude::*;

use sessions::{Request, Response, Session, SessionDiff};

use crate::constants::SESSION_ENDPOINT;
use crate::error::Error;
use crate::graphql::{quiz, Quiz, Round};
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
    ws: WebsocketTask,

    session: Option<Session>,
    data: Option<(Quiz, Vec<Round>)>,
}

impl Component for QuizLoader {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let url = format!("ws://{}/ws", SESSION_ENDPOINT);
        let mut ws = WebsocketTask::create(url, ctx.link().callback(Msg::WsResponse));

        match ctx.props().kind {
            Kind::Host { quiz_id } => {
                ctx.link().send_future(quiz(quiz_id).map(Msg::QuizLoaded));
                ws.send(&Request::Create { quiz_id })
            }
            Kind::Manage { session_id } => ws.send(&Request::Manage { session_id }),
        }

        Self { ws, session: None, data: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match (msg, ctx.props().kind) {
            (Msg::WsResponse(Response::Created(session)), Kind::Host { .. }) => {
                self.ws.send(&Request::Host { session_id: session.session_id });
                self.session = Some(session);

                if let Some(window) = window() {
                    window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
                }
                true
            }
            (Msg::WsResponse(Response::Updated(session)), _) => {
                if let (None, Kind::Manage { .. }) = (&self.data, ctx.props().kind) {
                    ctx.link().send_future(quiz(session.quiz_id).map(Msg::QuizLoaded));
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
            (Msg::Changed(SessionDiff { players: None, stage: None }), _) => false,
            (Msg::Changed(diff), _) => {
                if let Some(Session { session_id, .. }) = self.session {
                    self.ws.send(&Request::Update { session_id, diff });
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onchange = ctx.link().callback(Msg::Changed);

        match (&self.session, &self.data, ctx.props().kind) {
            (Some(session), Some((quiz, rounds)), Kind::Host { .. }) => {
                html! {<Host session={session.clone()} quiz={quiz.clone()} rounds={rounds.clone()} onchange={onchange} />}
            }
            (Some(session), Some((quiz, rounds)), Kind::Manage { .. }) => {
                html! {<Manage session={session.clone()} quiz={quiz.clone()} rounds={rounds.clone()} onchange={onchange} />}
            }
            _ => html! {},
        }
    }

    fn destroy(&mut self, _: &Context<Self>) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }
    }
}
