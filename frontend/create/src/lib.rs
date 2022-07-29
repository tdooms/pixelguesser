use cobul::Loader;
use shared::{Auth, Error, Errors, Route};
use yew::*;
use yew_router::prelude::Redirect;
use ywt::callback;

use crate::quiz::QuizPage;
use crate::rounds::RoundPage;
use crate::state::use_create_state;
use crate::summary::Summary;

mod edit;
mod list;
mod picker;
mod preview;
mod quiz;
mod rounds;
mod state;
mod summary;

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
    let onaction = callback!(state, user, errors; move |action| state.action(action, user.clone(), errors.clone()));

    let draft = state.quiz();
    let has_delete = props.quiz_id.is_some();

    let inner = match *stage {
        Stage::Quiz => html! { <QuizPage {onstage} {onaction} {draft} {has_delete}/> },
        Stage::Rounds => html! { <RoundPage {onstage} {onaction} {draft} /> },
        Stage::Summary => html! { <Summary {onstage} {draft} {onaction}/> },
        Stage::Back | Stage::Done => html! {<Redirect<Route> to={Route::Overview}/>},
    };

    match state.loading() {
        true => html! { <Loader /> },
        false => inner,
    }
}
