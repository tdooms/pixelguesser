use crate::agents::WebSocketAgent;
use crate::pages::host::InnerHost;
use api::{Post, Reply, Request, Response, Session, Stage};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HostLoaderProps {
    pub quiz_id: i64,
}

pub struct Host {
    props: HostLoaderProps,
    session: Option<(u64, Session)>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Component for Host {
    type Message = Response;
    type Properties = HostLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut ws_agent = WebSocketAgent::bridge(link.callback(|x| x));
        ws_agent.send(Request::Post(Post::StartSession { quiz_id: props.quiz_id }));

        Self { props, session: None, ws_agent }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Response::Reply(session_id, Reply::SessionCreated(quiz, rounds)) => {
                let session =
                    Session { stage: Stage::Initial, quiz, rounds, players: Default::default() };

                self.session = Some((session_id, session));
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
            Some((session_id, session)) => {
                html! {<InnerHost session={session.clone()} session_id={*session_id}/>}
            }
            None => html! {},
        }
    }
}
