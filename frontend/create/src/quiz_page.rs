use cobul::props::ColumnSize;
use cobul::*;
use shared::Auth;
use yew::prelude::*;
use yew::props;

use crate::quiz_form::QuizForm;
use api::{Creator, DraftQuiz, Resolution, IMAGE_PLACEHOLDER};
use components::QuizCard;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub quiz: DraftQuiz,

    #[prop_or_default]
    pub editing: bool,

    pub onsubmit: Callback<DraftQuiz>,
    pub oncancel: Callback<()>,
    pub ondelete: Callback<()>,
}

#[function_component(QuizPage)]
pub fn quiz_page(props: &Props) -> Html {
    let Props { onsubmit, oncancel, ondelete, quiz, editing } = &props;
    let state = use_state(move || quiz.clone());

    let DraftQuiz { title, public, description, image, .. } = (*state).clone();
    let image = image
        .as_ref()
        .map(|x| x.src(Resolution::Card))
        .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

    let creator: Creator = match use_context::<Auth>().unwrap().user() {
        Ok(user) => user.into(),
        Err(_) => return html! { "not allowed" },
    };

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
                    <QuizForm {form} editing={*editing}/>
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
