use cobul::Loading;
use futures::FutureExt;
use std::rc::Rc;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use yew_router::prelude::{History, RouterScopeExt};

use crate::{graphql, Auth, ErrorAgent, Route};
use sessions::{Action, Request, Response, Session};

use crate::graphql::FullQuiz;
use crate::pages::{Host, Manage};
use crate::shared::{Error, SESSION_ENDPOINT};
use crate::utils::WebsocketTask;

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
    Quiz(Result<FullQuiz, Error>),
    Action(Action),
}

impl Component for Loader {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let mut ws = WebsocketTask::create(SESSION_ENDPOINT, ctx.link().callback(Msg::Ws));

        let request = match ctx.props().session_id {
            Some(session_id) => Request::Manage(session_id),
            None => Request::Host,
        };

        let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
        ctx.link().send_future(graphql::quiz(auth, ctx.props().quiz_id).map(Msg::Quiz));
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
            Msg::Quiz(Err(err)) => {
                log::error!("{:?}", err);
                self.errors.send(err);
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
