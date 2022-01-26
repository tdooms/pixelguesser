use super::{Finish, Lobby, RoundComponent};
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
        Stage::Playing { round, paused, revealed } => {
            let url = rounds[round].image.clone();
            let players = session.players.clone();
            let rounds = rounds.len();

            html! { <RoundComponent {round} {rounds} {revealed} {paused} {url} {players}/> }
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
