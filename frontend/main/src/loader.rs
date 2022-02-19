use std::rc::Rc;

use cobul::Loading;
use futures::FutureExt;
use yew::prelude::*;

use yew_agent::{Dispatched, Dispatcher};
use yew_router::prelude::{History, RouterScopeExt};

use host::Host;
use manage::Manage;

use agents::{Auth, ErrorAgent};
use api::{Action, FullQuiz, Request, Response, Session, WebsocketTask};
use keys::SESSION_ENDPOINT;
use shared::{Error, Route};

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    // Having a session_id implies being a manager
    pub session_id: Option<u64>,
    pub quiz_id: u64,
}

pub struct Loader {
    ws: WebsocketTask,
    errors: Dispatcher<ErrorAgent>,

    session: Option<(u64, Rc<Session>)>,
    full: Option<Rc<FullQuiz>>,
}

pub enum Msg {
    Ws(Result<Response, api::Error>),
    Error(api::Error),
    Quiz(Result<FullQuiz, api::Error>),
    Action(Action),
}

impl Component for Loader {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let Props { session_id, quiz_id } = ctx.props().clone();

        let mut ws = WebsocketTask::create(
            SESSION_ENDPOINT,
            ctx.link().callback(Msg::Ws),
            ctx.link().callback(Msg::Error),
        );

        let request = match session_id {
            Some(session_id) => Request::Manage(session_id),
            None => Request::Host,
        };

        let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
        ctx.link().send_future(api::full_quiz(auth.into(), quiz_id).map(Msg::Quiz));
        ws.send(&request);

        let errors = ErrorAgent::dispatcher();
        Self { ws, errors, session: None, full: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quiz(Ok(full)) => self.full = Some(Rc::new(full)),
            Msg::Action(action) => {
                self.ws.send(&Request::Update(action, self.full.as_ref().unwrap().rounds.len()))
            }
            Msg::Ws(Ok(Response { id, managed: _, session })) => {
                // TODO: check if session with other id exists
                self.session = Some((id, Rc::new(session)));
            }
            Msg::Ws(Err(err)) => {
                self.errors.send(Error::Api(err));
                ctx.link().history().unwrap().push(Route::Overview)
            }
            Msg::Error(err) => {
                self.errors.send(Error::Api(err));
            }
            Msg::Quiz(Err(err)) => {
                self.errors.send(Error::Api(err));
                ctx.link().history().unwrap().push(Route::Overview)
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Loader { session, full, .. } = self;
        let callback = ctx.link().callback(Msg::Action);

        match (session.clone(), full.clone(), ctx.props().session_id) {
            (Some((_, session)), Some(full), Some(_)) => html! {
                <Manage {session} {full} {callback}/>
            },
            (Some((session_id, session)), Some(full), None) => html! {
                <Host {session} {session_id} {full} />
            },
            _ => html! {
                <Loading />
            },
        }
    }
}
