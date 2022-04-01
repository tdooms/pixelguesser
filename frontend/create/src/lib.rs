use log::info;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use shared::{callback, Auth, Errors, Route};

use crate::quiz_page::QuizPage;
use crate::round_page::RoundPage;
use crate::state::{use_create_state, CreateStage};
use crate::summary::Summary;

mod quiz_form;
mod quiz_page;
mod round_edit;
mod round_form;
mod round_list;
mod round_page;
mod round_preview;
mod state;
mod summary;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<u64>,
}

#[function_component(Create)]
pub fn create(props: &Props) -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user();
    let navigator = use_navigator().unwrap();
    let errors = use_context::<Errors>().unwrap();
    let state = use_create_state(props.id, user.clone().unwrap(), errors)?;

    let inner = match state.stage() {
        CreateStage::Quiz => {
            let callback = {
                let navigator = navigator.clone();
                move || navigator.push(Route::Overview)
            };

            let onsubmit = callback!(state; move |quiz| state.set_quiz(quiz));
            let onback = callback!(navigator; move |_| navigator.push(Route::Overview));
            let ondelete = callback!(state; move |_| state.delete(callback.clone()));

            let quiz = state.quiz();
            let editing = state.id().is_some();

            html! { <QuizPage {editing} {quiz} {onsubmit} {onback} {ondelete}/> }
        }
        CreateStage::Rounds => {
            let onsave = callback!(state; move |rounds| state.set_rounds(rounds));
            let ondone = callback!(state; move |_| state.set_stage(CreateStage::Summary));
            let onback = callback!(state; move |_| state.set_stage(CreateStage::Quiz));
            let rounds = state.rounds();
            log::info!("{:?}", rounds);

            html! { <RoundPage {rounds} {onsave} {ondone} {onback}/> }
        }
        CreateStage::Summary => {
            let ondone = callback!(navigator; move |_| navigator.push(Route::Overview));
            let onback = callback!(state; move |_| state.set_stage(CreateStage::Quiz));
            let (quiz, rounds) = (state.quiz(), state.rounds());

            html! { <Summary {quiz} {rounds} {ondone} {onback}/> }
        }
    };
    Ok(inner)
}
