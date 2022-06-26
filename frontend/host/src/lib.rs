mod finish;
mod info;
mod lobby;
mod play;
mod ranking;

use crate::finish::Finish;
use crate::lobby::Lobby;
use crate::play::Play;
use crate::ranking::Ranking;

use api::{Action, Code, FullQuiz, Phase, Session};
use js_sys::Function;
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub session_id: u32,
    pub session: Rc<Session>,
    pub full: Rc<FullQuiz>,
    pub callback: Callback<Action>,
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    use_effect_with_deps(
        move |_| {
            window().unwrap().set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")));
            || window().unwrap().set_onbeforeunload(None)
        },
        (),
    );

    let Props { session_id, session, full, callback } = props.clone();
    let code = Code { session_id, quiz_id: full.quiz.id as u32 }.to_string();

    let rounds = full.rounds.len();

    match session.phase {
        Phase::Lobby => html! {<Lobby {code} {session} {full} /> },
        Phase::Playing { round, stage } => {
            html! { <Play round={full.rounds[round].clone()} {rounds} {stage} players={session.players.clone()} {callback} index={round}/> }
        }
        Phase::Finished => html! {<Finish {session} {full} /> },
    }
}
