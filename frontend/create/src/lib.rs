use cobul::custom::Loading;
use shared::{Auth, Error, Errors, Route};
use yew::*;
use yew_router::prelude::Redirect;
use ywt::callback;

use crate::quiz_page::QuizPage;
use crate::quiz_summary::Summary;
use crate::round_page::RoundPage;
use crate::state::use_create_state;

mod quiz_form;
mod quiz_page;
mod quiz_summary;
mod round_edit;
mod round_form;
mod round_list;
mod round_page;
mod round_preview;
mod state;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz_id: Option<u32>,
}

pub enum Stage {
    Quiz,
    Rounds,
    Summary,

    Back,
    Done,
}

#[function_component(Create)]
pub fn create(props: &Props) -> Html {
    let user = use_context::<Auth>().unwrap().user();
    let errors = use_context::<Errors>().unwrap();

    let user = match user {
        Ok(user) => user,
        Err(_err) => return html! { <Redirect<Route> to={Route::Overview} /> },
    };

    let callback = callback!(errors; move |err| errors.emit(Error::Api(err)));
    let state = use_create_state(callback, props.quiz_id, Some(user.clone()), errors.clone());
    let stage = use_state(|| Stage::Quiz);

    let onstage = callback!(stage; move |new| stage.set(new));
    let onrounds = callback!(state, user, errors; move |action| state.set_rounds(action, user.clone(), errors.clone()));
    let onquiz = callback!(state, user, errors; move |action| state.set_quiz(action, user.clone(), errors.clone()));

    let inner = match *stage {
        Stage::Quiz => {
            html! { <QuizPage {onstage} onchange={onquiz} quiz={state.quiz()} has_delete={props.quiz_id.is_some()}/> }
        }
        Stage::Rounds => {
            html! { <RoundPage {onstage} onchange={onrounds} rounds={state.rounds()} /> }
        }
        Stage::Summary => {
            html! { <Summary {onstage} quiz={state.quiz()} rounds={state.rounds()} /> }
        }
        Stage::Back | Stage::Done => html! {<Redirect<Route> to={Route::Overview}/>},
    };

    match state.loading() {
        true => html! { <Loading /> },
        false => inner,
    }
}
