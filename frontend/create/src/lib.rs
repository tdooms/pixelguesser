mod quiz_form;
mod quiz_page;
mod round_edit;
mod round_form;
mod round_list;
mod round_page;
mod round_preview;
mod summary;

use crate::quiz_page::QuizPage;
use crate::round_edit::RoundEdit;
use crate::round_page::RoundPage;
use crate::summary::Summary;

use cobul::Loading;
use shared::{use_create_state, Auth, CreateStage, CreateState, Errors, Route};

use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<u64>,
}

#[function_component(Create)]
pub fn create(props: &Props) -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user();
    let errors = use_context::<Errors>().unwrap();
    let navigator = use_navigator().unwrap();
    let state = use_create_state(user.ok(), props.id)?;

    let show_state = |state: &CreateState| match state.stage() {
        CreateStage::Quiz => {
            let onsubmit = state.onsetquiz();
            let ondelete = state.ondelete();
            let onback = Callback::from(|_| navigator.push(Route::Overview));
            let quiz = state.quiz();

            html! { <QuizPage {quiz} {onsubmit} {ondelete} {onback}/> }
        }
        CreateStage::Rounds => {
            let onsave = state.onsetrounds();
            let ondone = state.onstage(CreateStage::Summary);
            let onback = state.onstage(CreateStage::Quiz);
            let rounds = state.rounds();

            html! { <RoundPage {rounds} {onsave} {ondone} {onback}/> }
        }
        CreateStage::Summary => {
            let ondone = Callback::from(|_| navigator.push(Route::Overview));
            let onback = state.onstage(CreateStage::Quiz);
            let (quiz, rounds) = (state.quiz(), state.rounds());

            html! { <Summary {quiz} {rounds} {ondone} {onback}/> }
        }
    };

    match &state {
        None => html! { <Loading /> },
        Some(state) => show_state(state),
    }
}
