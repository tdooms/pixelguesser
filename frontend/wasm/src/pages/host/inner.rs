use js_sys::Function;
use yew::prelude::*;
use yew::web_sys::window;
use yewtil::NeqAssign;

use crate::agents::WebSocketAgent;
use crate::components::Pixelate;
use crate::pages::host::{Finish, Lobby, Scores};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session_id: u64,
    pub session: Session,

    #[prop_or_default]
    pub has_manager: bool,

    pub onrevealed: Callback<()>,
}

pub struct InnerHost {
    link: ComponentLink<Self>,
    ws_agent: Box<dyn Bridge<WebSocketAgent>>,
    props: Props,
}

impl Component for InnerHost {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }

        let ws_agent = WebSocketAgent::bridge(link.callback(|x| Msg::Response(x)));
        Self { ws_agent, link, props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        self.props.onrevealed.emit(());

        match self.props.session.stage {
            Stage::Round { round, status: Status::Revealing } => {
                let stage = Stage::Round { round, status: Status::Revealed };
                let post = Post::ChangeStage { session_id: self.props.session_id, stage };
                self.ws_agent.send(Request::Post(post))
            }
            _ => {}
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.props.session.stage {
            Stage::Initial => {
                html! {
                    <Lobby session={self.props.session.clone()}
                        session_id={self.props.session_id}
                        has_manager={self.has_manager}
                    />
                }
            }
            Stage::Round { round, status } => {
                html! {
                    <Pixelate on_revealed={self.link.callback(|_| ())}
                        status={*status}
                        url={self.props.session.rounds[*round].image_url.clone()}
                    />
                }
            }
            Stage::Scores { round } => {
                html! {
                    <pbs::Section>
                        <pbs::Container>
                            <Scores players={self.props.session.players.clone()}/>
                        </pbs::Container>
                    </pbs::Section>
                }
            }
            Stage::Finish => {
                html! {
                    <Finish session_id={self.props.session_id}
                        players={self.props.session.players.clone()}
                        quiz={self.props.session.quiz.clone()}
                    />
                }
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }

        let session_id = self.props.session_id;
        let post = Post::StopSession { session_id };
        self.ws_agent.send(Request::Post(post))
    }
}
