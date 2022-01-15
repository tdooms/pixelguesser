use crate::graphql::{DraftRound, GuessChoices, PointChoices};
use cobul::props::{Alignment, Color};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Validate, Clone, Debug, PartialEq)]
pub struct RoundInfo {
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
}

impl From<&DraftRound> for RoundInfo {
    fn from(round: &DraftRound) -> Self {
        Self { answer: round.answer.clone(), points: round.points, guesses: round.guesses }
    }
}

#[function_component(RoundForm)]
pub fn round_form(props: &Form<RoundInfo>) -> Html {
    let errors = props.errors();
    let RoundInfo { answer, points, guesses } = &props.inner;

    html! {
        <div class="p-4">
        <SimpleField label="Answer" help={errors.get("answer").cloned()}>
            <Input oninput={props.field(|x| &mut x.answer)} value={answer.clone()}/>
        </SimpleField>

        <SimpleField label="Points" help={errors.get("points").cloned()}>
            <EnumButtons<PointChoices> onclick={props.field(|x| &mut x.points)} value={*points} color={Color::Link} alignment={Alignment::Centered} />
        </SimpleField>

        <SimpleField label="Guesses" help={errors.get("guesses").cloned()}>
            <EnumButtons<GuessChoices> onclick={props.field(|x| &mut x.guesses)} value={*guesses} color={Color::Link} alignment={Alignment::Centered} />
        </SimpleField>
        </div>
    }
}
