mod finish;
mod info;
mod lobby;
mod play;
mod ranking;

use crate::finish::Finish;
use crate::lobby::Lobby;
use crate::play::Play;
use crate::ranking::Ranking;

use api::{Code, FullQuiz, Session, Stage};
use js_sys::Function;
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub session_id: u64,
    pub session: Rc<Session>,
    pub quiz: Rc<FullQuiz>,
}

#[function_component(Host)]
pub fn host(props: &Props) -> Html {
    use_effect(move || {
        window().unwrap().set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")));

        || window().unwrap().set_onbeforeunload(None)
    });

    let Props { session_id, session, quiz } = props.clone();

    match session.stage {
        Stage::Lobby => {
            let code = Code { session_id, quiz_id: quiz.id }.to_string();
            html! { <Lobby {code} {session} {quiz}/> }
        }
        Stage::Playing { round, paused, revealing } => {
            html! { <Play index={round} {quiz} {session} {paused} {revealing}/> }
        }
        Stage::Finished => {
            html! { <Finish {session} {quiz}/> }
        }
    }
}
