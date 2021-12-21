use yew::prelude::*;

use sessions::{Request, Response, Session};
use crate::constants::SESSION_ENDPOINT;
use crate::graphql;

use crate::graphql::{Quiz, Round};
use crate::utils::yew::WebsocketTask;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Kind {
    Host { quiz_id: u64 },
    Manage { quiz_id: u64, session_id: u64 },
}

#[derive(PartialEq, Clone, Debug)]
struct Loader {
    ws: WebsocketTask,

    session: Option<(u64, Session)>,
    quiz: Option<(Quiz, Vec<Round>)>,
}

pub enum Msg {
    Ws(Response),
    Quiz(Result<(Quiz, Vec<Round>), ()>),
}

impl Component for Loader {
    type Message = Msg;
    type Properties = Kind;

    fn create(ctx: &Context<Self>) -> Self {
        let mut ws = WebsocketTask::create(SESSION_ENDPOINT, ctx.link().callback(Msg::Ws));

        match ctx.props() {
            Kind::Host {quiz_id} => {
                ws.send(&Request::Host);
                ctx.link().send_future(graphql::quiz(*quiz_id).map(Msg::Quiz))
            },
            Kind::Manage {quiz_id, session_id} => {
                ws.send(&Request::Manage(*session_id));
                ctx.link().send_future(graphql::quiz(*quiz_id).map(Msg::Quiz))
            }
        }

        Self { ws, session: None, quiz: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Quiz(Ok(pair)) => {
                self.quiz = Some(pair)
            }
            Msg::Ws(Response::Hosted())
        }

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}
