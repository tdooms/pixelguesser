use cobul::Loader;
use shared::{use_auth, use_toast, Forbidden, Route};
use yew::*;
use yew_router::prelude::Redirect;
use ywt::callback;

use crate::quiz::QuizPage;
use crate::round::RoundPage;
use crate::state::use_quiz_create;
use crate::summary::Summary;

mod edit;
mod list;
mod picker;
mod preview;
mod quiz;
mod round;
mod state;
mod summary;
mod unsplash;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz_id: Option<u64>,
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
    let auth = use_auth();
    let toast = use_toast();

    let zipped = auth.user().zip(auth.token());
    let (user, token) = match toast.maybe(zipped.ok_or(Forbidden)) {
        Some((user, token)) => (user, token),
        _ => return html! {},
    };

    let state = use_quiz_create(props.quiz_id);
    let stage = use_state(|| Stage::Quiz);

    let change = callback!(stage; move |new| stage.set(new));
    let action = callback!(state, user; move |action| state.action(action, token.clone(), user.id.clone().unwrap()));

    let quiz = state.quiz();

    let inner = match *stage {
        Stage::Quiz => html! { <QuizPage {change} {action} {quiz} /> },
        Stage::Rounds => html! { <RoundPage {change} {action} {quiz} /> },
        Stage::Summary => html! { <Summary {change} {action} {quiz} /> },
        Stage::Back | Stage::Done => html! {<Redirect<Route> to={Route::Overview}/>},
    };

    match state.loading() {
        true => html! { <Loader /> },
        false => inner,
    }
}
