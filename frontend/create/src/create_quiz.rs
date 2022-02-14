use agents::Auth;
use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;
use yew::props;

use crate::quiz_form::QuizForm;
use api::{Creator, DraftQuiz, Resolution};
use components::QuizCard;
use keys::IMAGE_PLACEHOLDER;

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

    let DraftQuiz { title, public, description, image, .. } = (*state).clone();
    let image = image
        .as_ref()
        .map(|x| x.src(Resolution::Card))
        .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

    let creator: Creator = match use_context::<Auth>().unwrap() {
        Auth::User(user) => user.into(),
        Auth::Loading | Auth::Anonymous => return html! { "not allowed" },
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
                    <QuizForm form={form} editing={quiz.is_some()}/>
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
