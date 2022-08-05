use cobul::Loader;
use shared::{use_auth, use_toast, Forbidden, Route};
use yew::*;
use yew_router::prelude::Redirect;
use ywt::callback;

use crate::quiz::QuizPage;
use crate::rounds::RoundPage;
use crate::state::use_quiz_create;
use crate::summary::Summary;

mod edit;
mod list;
mod picker;
mod preview;
mod quiz;
mod rounds;
mod state;
mod summary;
mod unsplash;

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
    let user = use_auth().user();
    let toast = use_toast();

    let user = match toast.maybe(user.ok_or(Forbidden)) {
        Some(user) => user,
        None => return html! {},
    };

    let state = use_quiz_create(props.quiz_id);
    let stage = use_state(|| Stage::Quiz);

    let onstage = callback!(stage; move |new| stage.set(new));
    let onaction = callback!(state, user; move |action| state.action(action, user.clone()));

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
