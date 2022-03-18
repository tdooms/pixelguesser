use cobul::props::{Color, ColumnSize};
use cobul::*;
use yew::prelude::*;

use api::{Creator, DraftQuiz, Resolution, IMAGE_PLACEHOLDER};
use components::QuizCard;
use shared::{callback, Auth};

use crate::quiz_form::QuizForm;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub quiz: DraftQuiz,
    pub editing: bool,

    pub onsubmit: Callback<DraftQuiz>,
    pub onback: Callback<()>,
    pub ondelete: Callback<()>,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { onsubmit, onback, ondelete, quiz, editing } = props.clone();

    let state = use_state(|| quiz.clone());
    let DraftQuiz { title, public, description, image, .. } = (*state).clone();

    let image = image
        .as_ref()
        .map(|x| x.src(Resolution::Card))
        .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

    let creator: Creator = match use_context::<Auth>().unwrap().user() {
        Ok(user) => user.into(),
        Err(_) => return html! { "not allowed" },
    };

    let onchange = callback!(state; move |quiz| state.set(quiz));

    let left = html! {<Title> {"Overview"} </Title>};

    let right = match editing {
        false => {
            html! {<Button color={Color::Danger} onclick={ondelete}> {"Delete Quiz"} </Button>}
        }
        true => html! {},
    };

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
