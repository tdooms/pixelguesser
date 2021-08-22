use shared::{Request, Response, Session};
use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::pages::host::InnerHost;
use crate::utils::{code_to_string, WebsocketTask};
use graphql::{Quiz, Round};

pub enum Msg {
    WsResponse(Response),
    Revealed,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HostLoaderProps {
    pub quiz_id: u64,
}

pub struct Host {
    props: HostLoaderProps,
    link: ComponentLink<Self>,

    ws: WebsocketTask,

    session: Option<(u64, Session)>,
    quiz_data: Option<(Quiz, Vec<Round>)>,
}

impl Component for Host {
    type Message = Msg;
    type Properties = HostLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: create session
        // TODO: host session
        // TODO: fetch quizzes
        let ws = WebsocketTask::create("TODO", link.callback(Msg::WsResponse));
        ws.send(Request::Create);

        Self { props, link, ws, session: None, quiz_data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsResponse(Response::Created(session_id)) => {
                // TODO: store session_id
                self.ws.send(Request::Host(session_id));
                true
            }
            Msg::WsResponse(Response::Updated(session)) => {
                // TODO: set session
                true
            }
            Msg::WsResponse(_) => {
                log::error!("unexpected response");
                false
            }
            Msg::Revealed => {
                // self.ws.send(Request::Update())
                // TODO: change stage of session
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match (&self.session, &self.quiz_data) {
            (Some((session_id, session)), Some((quiz, rounds))) => {
                let onrevealed = self.link.callback(|_| Msg::Revealed);
                let code = code_to_string(*session_id).unwrap_or_default();

                html! {<InnerHost code={code} session={session.clone()} onrevealed={onrevealed} quiz={quiz.clone()} rounds={rounds.clone()}/> }
            }
            _ => html! {},
        }
    }
}
