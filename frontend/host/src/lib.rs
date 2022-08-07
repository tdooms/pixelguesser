mod finish;
mod info;
mod lobby;
mod play;
mod ranking;

use crate::finish::Finish;
use crate::lobby::Lobby;
use crate::play::Play;
use crate::ranking::Ranking;

use api::{Action, Code, Phase, Quiz, Session};
use shared::use_toast;
use std::rc::Rc;
use yew::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub session_id: u32,
    pub session: Rc<Session>,
    pub quiz: Rc<Quiz>,

    pub callback: Callback<Action>,
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    let Props { session_id, session, quiz, callback } = props.clone();
    let code = Code { session_id, quiz_id: quiz.id as u32 }.to_string();

    let toast = use_toast();
    let rounds = quiz.rounds.len();

    match session.phase {
        Phase::Lobby => {
            html! {<Lobby {code} {session} {quiz} /> }
        }
        Phase::Playing { round, .. } if round > rounds => {
            toast.warning("This quiz is empty", true);
            html! {}
        }
        Phase::Playing { round: index, stage } => {
            let round = quiz.rounds[index].clone();
            let players = session.players.clone();
            html! { <Play {round} {rounds} {stage} {players} {callback} {index}/> }
        }
        Phase::Finished => {
            html! {<Finish {session} {quiz} /> }
        }
    }
}
