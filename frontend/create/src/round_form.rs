use api::{DraftRound, GuessChoices, PointChoices};
use cobul::props::{Alignment, Color, Size};
use cobul::*;
use validator::Validate;
use yew::prelude::*;

#[derive(Validate, Clone, Debug, PartialEq)]
pub struct RoundInfo {
    #[validate(length(min = 1, message = "Must not be empty"))]
    pub answer: String,
    pub points: PointChoices,
    pub guesses: GuessChoices,
}

impl From<DraftRound> for RoundInfo {
    fn from(round: DraftRound) -> Self {
        Self { answer: round.answer.clone(), points: round.points, guesses: round.guesses }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<RoundInfo>,
    pub onremove: Callback<()>,
    pub info: RoundInfo,
}

#[function_component(RoundForm)]
pub fn round_form(props: &Props) -> Html {
    let actions = Actions::new().change(props.onchange.clone());
    let (form, info) = use_form(&props.info, actions);

    let RoundInfo { answer, points, guesses } = info;

    html! {
        <div class="pt-5 pl-4 pr-5">
        <SimpleField label="Answer" help={form.error("answer")} >
            <Input oninput={form.field(|x| &mut x.answer)} value={answer}/>
        </SimpleField>

        <SimpleField label="Points" help={form.error("points")} >
            <EnumButtons<PointChoices> onclick={form.field(|x| &mut x.points)} value={points}
            color={Color::Link} alignment={Alignment::Centered} size={Size::Small}/>
        </SimpleField>

        <SimpleField label="Guesses" help={form.error("guesses")}>
            <EnumButtons<GuessChoices> onclick={form.field(|x| &mut x.guesses)} value={guesses}
            color={Color::Link} alignment={Alignment::Centered} size={Size::Small}/>
        </SimpleField>
        <Button fullwidth=true onclick={&props.onremove}> {"Remove image"} </Button>
        </div>
    }
}
