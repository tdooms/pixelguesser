use js_sys::Function;
use pbs::prelude::*;
use yew::prelude::*;
use yew::utils::NeqAssign;
use yew::web_sys::window;

use graphql::{Quiz, Round};
use shared::{Session, Stage};

use crate::components::Pixelate;
use crate::pages::host::{Finish, Lobby, Scores};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,
    pub code: String,

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
        let Props { quiz, rounds, session, code, .. } = &self.props;

        match &session.stage {
            Stage::Initial => {
                html! { <Lobby code={code.clone()} session={session.clone()} quiz={quiz.clone()}/> }
            }
            Stage::Round { round, status } => {
                let onrevealed = self.props.onrevealed.reform(|x| x);
                let url = rounds[*round].image_url.clone();

                html! { <Pixelate onrevealed={onrevealed} status={*status} url={url}/> }
            }
            Stage::Ranking { round } => {
                html! {
                    <Section>
                        <Container>
                            <Scores players={session.players.clone()}/>
                        </Container>
                    </Section>
                }
            }
            Stage::Finished => {
                html! { <Finish players={session.players.clone()} quiz={quiz.clone()}/> }
            }
        }
    }

    fn destroy(&mut self) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }
    }
}
