use yew::prelude::*;

use shared::{callback, Auth, Errors, Route};

use crate::quiz_page::QuizPage;
use crate::round_page::RoundPage;
use crate::state::{use_create_state, CreateStage, UseCreateStateHandle};
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
    let errors = use_context::<Errors>().unwrap();

    // log::info!("create main page before");
    let state = use_create_state(props.id, user.unwrap(), errors)?;
    // log::info!("create main page after");

    let inner = match state.stage() {
        CreateStage::Quiz => html! { <QuizPage/> },
        CreateStage::Rounds => html! { <RoundPage/> },
        CreateStage::Summary => html! { <Summary/> },
    };

    Ok(html! {
        <ContextProvider<UseCreateStateHandle> context={state}>
            {inner}
        </ContextProvider<UseCreateStateHandle>>
    })
}
