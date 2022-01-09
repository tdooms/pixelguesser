use cobul::props::ColumnSize;
use cobul::*;
use yew::prelude::*;

use super::QuizForm;
use crate::components::QuizCard;
use crate::constants::IMAGE_PLACEHOLDER;
use crate::graphql::DraftQuiz;
use crate::structs::ImageData;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<DraftQuiz>,
    pub oncancel: Callback<()>,
}

#[function_component(CreateQuiz)]
pub fn quiz_form(props: &Props) -> Html {
    let Props { onsubmit, oncancel } = &props;

    let state = use_state(|| DraftQuiz::default());
    let DraftQuiz { name, creator, description, image } = (*state).clone();
    let src = image.as_ref().map(ImageData::src).unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

    html! {
        <Columns>
            <Column>
                <QuizForm inner={(*state).clone()} onsubmit={onsubmit} oncancel={oncancel.reform(|_| ())}/>
            </Column>
            <Column size={ColumnSize::Is4}>
                <QuizCard name={name} creator={creator} description={description} src={src}/>
            </Column>
        </Columns>
    }
}
