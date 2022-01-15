use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;
use yew::props;

use super::QuizForm;
use crate::components::QuizCard;
use crate::constants::IMAGE_PLACEHOLDER;
use crate::graphql::DraftQuiz;
use crate::structs::ImageData;

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

    let DraftQuiz { name, creator, description, image } = (*state).clone();
    let src = image.as_ref().map(ImageData::src).unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

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
                    <QuizCard name={name} creator={creator} description={description} image={src}/>
                </Column>
            </Columns>
        </Container>
        </Section>
    }
}
