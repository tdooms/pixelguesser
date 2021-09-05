use yew::prelude::*;

use pbs::prelude::*;
use shared::{Session, SessionDiff, Stage, Status};

use crate::components::Pixelate;
use crate::graphql::{Quiz, Round};
use crate::pages::host::{Finish, Lobby, Scores};
use crate::utils::misc::code_to_string;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub session: Session,
    pub quiz: Quiz,
    pub rounds: Vec<Round>,

    pub onchange: Callback<SessionDiff>,
}

pub fn change_stage(stage: Stage) -> Option<Stage> {
    match stage {
        Stage::Round { round, .. } => Some(Stage::Round { round, status: Status::Revealed }),
        _ => None
    }
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    let Props { quiz, rounds, session, onchange } = &props;

    match &session.stage {
        Stage::Initial => {
            let code = code_to_string(session.session_id).unwrap_or_default();
            html! { <Lobby code={code.clone()} session={session.clone()} quiz={quiz.clone()}/> }
        }
        Stage::Round { round, status } => {
            let stage = change_stage(session.stage);
            let onrevealed = onchange.reform(move |_| SessionDiff { stage, players: None });

            let url = rounds[*round].image_url.clone().unwrap();

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