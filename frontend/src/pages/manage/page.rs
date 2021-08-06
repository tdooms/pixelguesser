use api::{Post, Reply, Request, Response, Session};
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::agents::WebSocketAgent;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ManageLoaderProps {
    pub session_id: u64,
}

pub struct Manage {
    props: ManageLoaderProps,
    session: Option<Session>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Component for Manage {
    type Message = Response;
    type Properties = ManageLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| x));

        let session_id = props.session_id;
        let request = Request::Post(Post::JoinSession { session_id });
        ws_agent.send(request);

        Self { props, session: None, ws_agent }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Response::Reply(session_id, Reply::SessionJoined(session)) => {
                self.session = Some(session);
                true
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.session {
            Some(session) => {
                html! {<Manage session={session.clone()} session_id={self.props.session_id}/>}
            }
            None => html! {},
        }
    }
}
