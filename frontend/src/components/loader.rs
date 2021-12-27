use cobul::Loading;
use futures::FutureExt;
use yew::prelude::*;

use sessions::{Action, Request, Response, Session};

use crate::constants::SESSION_ENDPOINT;
use crate::error::Error;
use crate::graphql;
use crate::graphql::{Quiz, Round};
use crate::pages::{Host, Manage};
use crate::utils::WebsocketTask;

#[derive(Properties, Clone, Debug, PartialEq, Copy)]
pub struct Props {
    pub quiz_id: u64,
    pub session_id: Option<u64>, // Having a session_id implies being a manager
}

pub struct Loader {
    ws: WebsocketTask,

    session: Option<(u64, Session)>,
    quiz: Option<(Quiz, Vec<Round>)>,
}

pub enum Msg {
    Ws(Response),
    Quiz(Result<(Quiz, Vec<Round>), Error>),
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

        ctx.link().send_future(graphql::quiz(ctx.props().quiz_id).map(Msg::Quiz));
        ws.send(&request);

        Self { ws, session: None, quiz: None }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quiz(Ok(pair)) => self.quiz = Some(pair),
            Msg::Action(action) => {
                self.ws.send(&Request::Update(action, self.quiz.as_ref().unwrap().1.len()))
            }
            Msg::Ws(Response::Hosted(id, session)) => self.session = Some((id, session)),
            Msg::Ws(Response::Managed(id, session)) => self.session = Some((id, session)),
            Msg::Ws(Response::Updated(session)) => {
                // TODO: some checks
                self.session.as_mut().unwrap().1 = session;
            }
            Msg::Ws(Response::Error(err)) => {
                log::error!("{:?}", err)
            }
            Msg::Quiz(Err(err)) => {
                log::error!("{:?}", err)
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Loader { session, quiz, .. } = self;
        let callback = ctx.link().callback(Msg::Action);

        match (session.clone(), quiz.clone(), ctx.props().session_id) {
            (Some((_, session)), Some((quiz, rounds)), Some(_)) => html! {
                <Manage session={session} quiz={quiz} rounds={rounds} callback={callback}/>
            },
            (Some((session_id, session)), Some((quiz, rounds)), None) => html! {
                <Host session={session} session_id={session_id} quiz={quiz} rounds={rounds} />
            },
            _ => html! {
                <Loading />
            },
        }
    }
}
