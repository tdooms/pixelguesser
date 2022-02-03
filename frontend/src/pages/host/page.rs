use super::{Finish, Lobby, RoundPlay};
use crate::graphql::FullQuiz;
use crate::utils::code_to_string;
use js_sys::Function;
use sessions::{Session, Stage};
use std::rc::Rc;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub session_id: u64,
    pub session: Rc<Session>,
    pub quiz: Rc<FullQuiz>,
}

pub struct Host;

impl Component for Host {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        if let Some(window) = window() {
            window.set_onbeforeunload(Some(&Function::new_with_args("", "return 'no'")))
        }
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { session_id, session, quiz } = ctx.props().clone();

        match session.stage {
            Stage::Lobby => {
                let code = code_to_string(session_id, quiz.id).unwrap_or_default();
                html! { <Lobby {code} {session} {quiz}/> }
            }
            Stage::Playing { round, paused, revealing } => {
                html! { <RoundPlay index={round} {quiz} {session} {paused} {revealing}/> }
            }
            Stage::Finished => {
                html! { <Finish {session} {quiz}/> }
            }
        }
    }

    fn destroy(&mut self, _: &Context<Self>) {
        if let Some(window) = window() {
            window.set_onbeforeunload(None)
        }
    }
}
