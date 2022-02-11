use std::rc::Rc;

use cobul::Loading;
use futures::FutureExt;
use reqwasm::websocket::WebSocketError;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use yew_router::prelude::{History, RouterScopeExt};

use sessions::{Action, Request, Response, Session};

use host::Host;
use manage::Manage;

use crate::shared::Error;
use crate::utils::WebsocketTask;
use crate::{Auth, ErrorAgent, Route};
use api::FullQuiz;
use keys::SESSION_ENDPOINT;

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    pub quiz_id: u64,
    pub session_id: Option<u64>, // Having a session_id implies being a manager
}

pub struct Loader {
    ws: WebsocketTask<Request, Result<Response, sessions::Error>>,
    errors: Dispatcher<ErrorAgent>,

    session: Option<(u64, Rc<Session>)>,
    quiz: Option<Rc<FullQuiz>>,
}

pub enum Msg {
    Ws(Result<Response, sessions::Error>),
    Error(WebSocketError),
    Quiz(Result<FullQuiz, api::Error>),
    Action(Action),
}

impl Component for Loader {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut ws = WebsocketTask::create(
            SESSION_ENDPOINT,
            ctx.link().callback(Msg::Ws),
            ctx.link().callback(Msg::Error),
        );

        let request = match ctx.props().session_id {
            Some(session_id) => Request::Manage(session_id),
            None => Request::Host,
        };

        let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
        ctx.link().send_future(api::full_quiz(auth.into(), ctx.props().quiz_id).map(Msg::Quiz));
        ws.send(&request);

        let errors = ErrorAgent::dispatcher();
        Self { ws, errors, session: None, quiz: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quiz(Ok(quiz)) => self.quiz = Some(Rc::new(quiz)),
            Msg::Action(action) => {
                self.ws.send(&Request::Update(action, self.quiz.as_ref().unwrap().rounds.len()))
            }
            Msg::Ws(Ok(Response { id, managed, session })) => {
                // TODO: check if session with other id exists
                self.session = Some((id, Rc::new(session)));
            }
            Msg::Ws(Err(err)) => {
                self.errors.send(Error::Session(err));
                ctx.link().history().unwrap().push(Route::Overview)
            }
            Msg::Error(WebSocketError::ConnectionClose(ev)) => {
                self.errors.send(Error::WebSocket("connection closed".to_owned()));
                ctx.link().history().unwrap().push(Route::Overview)
            }
            Msg::Error(err) => {
                self.errors.send(Error::WebSocket("websocket error".to_owned()));
            }
            Msg::Quiz(Err(err)) => {
                self.errors.send(Error::Api(err));
                ctx.link().history().unwrap().push(Route::Overview)
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Loader { session, quiz, .. } = self;
        let callback = ctx.link().callback(Msg::Action);

        match (session.clone(), quiz.clone(), ctx.props().session_id) {
            (Some((_, session)), Some(quiz), Some(_)) => html! {
                <Manage {session} {quiz} {callback}/>
            },
            (Some((session_id, session)), Some(quiz), None) => html! {
                <Host {session} {session_id} {quiz} />
            },
            _ => html! {
                <Loading />
            },
        }
    }
}
