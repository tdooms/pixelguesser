use super::{Finish, Lobby, Pixelate, Ranking};
use crate::graphql::{Quiz, Round};
use crate::utils::misc::code_to_string;
use sessions::{Session, Stage};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
struct Props {
    session: Session,
    secret: u64,

    quiz: Quiz,
    rounds: Vec<Round>,
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    let Props { session, secret, quiz, rounds } = props;

    match session.stage {
        Stage::Lobby => {
            let code = code_to_string(secret).unwrap_or_default();
            html! { <Lobby code={code.clone()} session={session.clone()} quiz={quiz.clone()}/> }
        }
        Stage::Playing { round, paused } => {
            let url = rounds[round].image_url.clone().unwrap();
            html! { <Pixelate revealing={false} paused={paused} url={url}/> }
        }
        Stage::Revealed { round } => {
            let url = rounds[round].image_url.clone().unwrap();
            html! { <Pixelate revealing={true} paused={false} url={url}/> }
        }
        Stage::Ranking { round } => {
            html! { <Ranking players={session.players.clone()}/> }
        }
        Stage::Finished => {
            html! { <Finish players={session.players.clone()} quiz={quiz.clone()}/> }
        }
    }
}
