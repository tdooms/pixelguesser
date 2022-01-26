use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;
use yew::props;

use super::QuizForm;
use crate::components::QuizCard;
use crate::graphql::{Creator, DraftQuiz, ImageData};
use crate::shared::{User, IMAGE_PLACEHOLDER};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub quiz: Option<DraftQuiz>,

    pub onsubmit: Callback<DraftQuiz>,
    pub oncancel: Callback<()>,
    pub ondelete: Callback<()>,
}

#[function_component(CreateQuiz)]
pub fn quiz_form(props: &Props) -> Html {
    let Props { onsubmit, oncancel, ondelete, quiz } = &props;
    let state = use_state(move || quiz.clone().unwrap_or_default());

    let DraftQuiz { title, public, explanation, description, image } = (*state).clone();
    let image = image.as_ref().map(ImageData::src).unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

    let creator: Creator = use_context::<User>().unwrap().into();

    let onchange = {
        let cloned = state.clone();
        Callback::from(move |quiz| cloned.set(quiz))
    };

    let form = props!(Form<DraftQuiz> {
        inner: (*state).clone(),
        onsubmit,
        onchange,
        oncancel: oncancel.reform(|_| ()),
        onreset: ondelete.reform(|_| ())
    });

    html! {
        <Section>
        <Container>
            <Columns>
                <Column>
                    <QuizForm form={form} editing={quiz.is_some()}/>
                </Column>
                <Column size={ColumnSize::Is4}>
                    <QuizCard {title} {description} {image} {creator}/>
                </Column>
            </Columns>
        </Container>
        </Section>
    }
}
