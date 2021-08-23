use futures::FutureExt;
use yew::prelude::*;
use yew::utils::NeqAssign;

use shared::{Request, Response, Session, SessionDiff, Stage, Status};

use crate::constants::SESSION_ENDPOINT;
use crate::error::Error;
use crate::graphql::{quiz, Quiz, Round};
use crate::pages::host::InnerHost;
use crate::utils::misc::code_to_string;
use crate::utils::yew::WebsocketTask;

pub enum Msg {
    WsResponse(Response),
    Quiz(Result<(Quiz, Vec<Round>), Error>),
    Revealed,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub quiz_id: u64,
}

pub struct Host {
    props: Props,
    link: ComponentLink<Self>,

    ws: WebsocketTask,

    session: Option<Session>,
    data: Option<(Quiz, Vec<Round>)>,
}

impl Component for Host {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let url = format!("ws://{}/ws", SESSION_ENDPOINT);
        let mut ws = WebsocketTask::create(url, link.callback(Msg::WsResponse));

        ws.send(&Request::Create { quiz_id: props.quiz_id });
        link.send_future(quiz(props.quiz_id).map(Msg::Quiz));

        Self { props, link, ws, session: None, data: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match (msg, &mut self.session) {
            (Msg::WsResponse(Response::Created(session)), None) => {
                self.ws.send(&Request::Host { session_id: session.session_id });
                self.session = Some(session);
                true
            }
            (Msg::WsResponse(Response::Updated(session)), Some(old)) => {
                *old = session;
                true
            }
            (Msg::WsResponse(_), _) => {
                log::error!("unexpected response");
                false
            }
            (Msg::Quiz(Ok(pair)), _) => {
                self.data = Some(pair);
                true
            }
            (Msg::Quiz(Err(err)), _) => {
                log::error!("{:?}", err);
                false
            }
            (Msg::Revealed, Some(session)) => {
                let stage = match session.stage {
                    Stage::Round { round, .. } => Stage::Round { round, status: Status::Revealed },
                    _ => return false // TODO: give error
                };
                let diff = SessionDiff { stage: Some(stage), players: vec![] };
                self.ws.send(&Request::Update { session_id: session.session_id, diff });
                false
            }
            _ => {
                log::error!("invalid config");
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let view_page = |session_id: u64, session: &Session, quiz: &Quiz, rounds: &[Round]| {
            let onrevealed = self.link.callback(|_| Msg::Revealed);
            let code = code_to_string(session_id).unwrap_or_default();

            html! {<InnerHost code={code} session={session.clone()} onrevealed={onrevealed} quiz={quiz.clone()} rounds={rounds.to_vec()}/> }
        };

        match (&self.session, &self.data) {
            (Some((session_id, session)), Some((quiz, rounds))) => {
                view_page(*session_id, session, quiz, rounds)
            }
            _ => html! { <cbs::Loading /> },
        }
    }
}
