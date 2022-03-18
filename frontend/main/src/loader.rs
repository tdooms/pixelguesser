use std::rc::Rc;

use cobul::Loading;
use yew::prelude::*;

use yew_router::prelude::RouterScopeExt;

use host::Host;
use manage::Manage;

use api::{Action, FullQuiz, Request, Response, Session, WebsocketTask, SESSION_ENDPOINT};
use shared::{async_callback, Auth, Error, Errors, Route};

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    // Having a session_id implies being a manager
    pub session_id: Option<u64>,
    pub quiz_id: u64,
}

pub struct Loader {
    ws: WebsocketTask,
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
        async_callback(api::full_quiz(auth.user().ok(), quiz_id), ctx.link().callback(Msg::Quiz));
        ws.send(&request);

        Self { ws, session: None, full: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (errors, _) = ctx.link().context::<Errors>(Callback::noop()).unwrap();
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
                errors.emit(Error::Api(err));
                ctx.link().navigator().unwrap().push(Route::Overview)
            }
            Msg::Error(err) => {
                errors.emit(Error::Api(err));
            }
            Msg::Quiz(Err(err)) => {
                errors.emit(Error::Api(err));
                ctx.link().navigator().unwrap().push(Route::Overview)
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
