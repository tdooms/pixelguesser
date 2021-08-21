use js_sys::Function;
use yew::prelude::*;
use yew::utils::NeqAssign;
use yew::web_sys::window;

use crate::components::Pixelate;
use crate::pages::host::{Finish, Lobby, Scores};
use shared::{Stage, Session};
use futures::StreamExt;
use graphql::{Quiz, Round};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,

    pub onrevealed: Callback<()>,
}

pub struct InnerHost {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for InnerHost {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }

        Self { link, props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.props.session.stage {
            Stage::Initial => {
                html! { <Lobby session={self.props.session.clone()} quiz={self.props.quiz}/> }
            }
            Stage::Round { round, status } => {
                let onrevealed = self.props.onrevealed.reform(|x| x);
                let url = self.props.rounds[*round].image_url.clone();
                    
                html! { <Pixelate onrevealed={onrevealed} status={*status} url={url}/> }
            }
            Stage::Ranking { round } => {
                html! {
                    <pbs::Section>
                        <pbs::Container>
                            <Scores players={self.props.session.players.clone()}/>
                        </pbs::Container>
                    </pbs::Section>
                }
            }
            Stage::Finished => {
                html! { <Finish players={self.props.session.players.clone()} quiz={self.props.quiz.clone()}/> }
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }
        // TODO: kill session?
    }
}
