use super::{Finish, Lobby, Ranking};
use crate::components::Pixelate;
use crate::graphql::{Quiz, Round};
use crate::shared::Route;
use crate::utils::code_to_string;
use sessions::{Session, Stage};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub session: Session,
    pub session_id: u64,

    pub quiz: Quiz,
    pub rounds: Vec<Round>,
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    let Props { session, session_id, quiz, rounds } = props;

    match session.stage {
        Stage::Lobby => {
            let code = code_to_string(*session_id, quiz.id).unwrap_or_default();
            html! { <Lobby code={code.clone()} session={session.clone()} quiz={quiz.clone()}/> }
        }
        Stage::Playing { round, paused } => {
            let url = rounds[round].image.clone();
            html! { <Pixelate revealing={false} paused={paused} url={url}/> }
        }
        Stage::Revealed { round } => {
            let url = rounds[round].image.clone();
            html! { <Pixelate revealing={true} paused={false} url={url}/> }
        }
        Stage::Ranking { .. } => {
            html! { <Ranking players={session.players.clone()}/> }
        }
        Stage::Finished => {
            html! { <Finish players={session.players.clone()} quiz={quiz.clone()}/> }
        }
        Stage::Left => {
            use_history().unwrap().push(Route::Overview);
            html! {}
        }
    }
}
