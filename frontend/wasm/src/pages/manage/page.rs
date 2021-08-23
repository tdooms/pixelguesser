use yew::prelude::*;
use yew::utils::NeqAssign;

use shared::{Request, Response, Session};

use crate::graphql::{Quiz, Round};
use crate::pages::manage::InnerManage;
use crate::utils::yew::WebsocketTask;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ManageLoaderProps {
    pub session_id: u64,
}

pub enum Msg {
    Changed(Session),
    WsResponse(Response),
}

pub struct Manage {
    props: ManageLoaderProps,
    link: ComponentLink<Self>,

    ws: WebsocketTask,

    session: Option<Session>,
    quiz_data: Option<(Quiz, Vec<Round>)>,
}

impl Component for Manage {
    type Message = Msg;
    type Properties = ManageLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ws = WebsocketTask::create("TODO", link.callback(Msg::WsResponse));
        ws.send(Request::Manage { session_id: props.session_id });

        Self { props, link, ws, session: None, quiz_data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsResponse(Response::Updated(session)) => {
                self.session = Some(session);
                true
            }
            Msg::WsResponse(_) => {
                log::error!("unknown response");
                false
            }
            Msg::Changed(session) => {
                self.session = Some(session.clone());
                self.ws.send(Request::Update { session_id: self.props.session_id, session });
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match (&self.session, &self.quiz_data) {
            (Some(session), Some((quiz, rounds))) => {
                let onchange = self.link.callback(Msg::Changed);
                html! {<InnerManage session={session.clone()} quiz={quiz.clone()} rounds={rounds.clone()} onchange={onchange} />}
            }
            _ => html! {},
        }
    }
}
