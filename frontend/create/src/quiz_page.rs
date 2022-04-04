use cobul::props::{Color, ColumnSize};
use cobul::*;
use yew::prelude::*;
use yew_router::prelude::use_navigator;

use api::{Creator, DraftQuiz, Image, Resolution, IMAGE_PLACEHOLDER};
use components::QuizCard;
use shared::{callback, Auth};

use crate::quiz_form::QuizForm;
use crate::state::UseCreateStateHandle;
use crate::Route;

#[function_component(QuizPage)]
pub fn quiz_page() -> Html {
    let state = use_context::<UseCreateStateHandle>().unwrap();
    let navigator = use_navigator().unwrap();

    let quiz = state.quiz();
    let DraftQuiz { title, description, image, public, .. } = quiz.clone();

    log::trace!("quiz page render {:?}", quiz);

    let image = Image::src_or_placeholder(image.as_ref(), Resolution::Card);

    let creator: Creator = match use_context::<Auth>().unwrap().user() {
        Ok(user) => user.into(),
        Err(_) => return html! { "not allowed" },
    };

    let callback = {
        let navigator = navigator.clone();
        move || navigator.push(Route::Overview)
    };

    let onchange = callback!(state; move |quiz| state.set_quiz(quiz));
    let onsubmit = callback!(state; move |_| state.submit_quiz());
    let onback = callback!(navigator; move |_| navigator.push(Route::Overview));
    let ondelete = callback!(state, navigator; move |_| state.delete(callback.clone()));

    let delete = || html! {<Button color={Color::Danger} onclick={ondelete}> {"Delete"} </Button>};
    let left = html! {<Title> {"Overview"} </Title>};
    let right = state.id().map(|_| delete()).unwrap_or_default();

    html! {
        <Section>
        <Container>
            <Columns>
                <Column>
                    <Level {left} {right} />
                    <QuizForm {quiz} {onsubmit} {onback} {onchange}/>
                </Column>
                <Column size={ColumnSize::Is1} />
                <Column size={ColumnSize::Is4}>
                    <QuizCard {title} {description} {image} {creator} {public}/>
                </Column>
            </Columns>
        </Container>
        </Section>
    }
}
