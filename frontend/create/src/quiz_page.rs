use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;
use yew::props;

use api::{Creator, DraftQuiz, Resolution, IMAGE_PLACEHOLDER};
use components::QuizCard;
use shared::{callback, Auth};

use crate::quiz_form::QuizForm;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub quiz: Option<DraftQuiz>,

    pub onsubmit: Callback<DraftQuiz>,
    pub onback: Callback<()>,
    pub ondelete: Callback<()>,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { onsubmit, onback, ondelete, quiz } = props.clone();

    let state = use_state(|| DraftQuiz::default());
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

    html! {
        <Section>
        <Container>
            <Columns>
                <Column>
                    <QuizForm {quiz} {onsubmit} {onback} {onchange} {ondelete}/>
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
