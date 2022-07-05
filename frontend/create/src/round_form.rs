use api::{DraftRound, GuessChoices, PointChoices};
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
    pub draft: DraftRound,
}

#[function_component(RoundForm)]
pub fn round_form(props: &Props) -> Html {
    let has_image = !props.draft.image.is_none();
    let info: RoundInfo = props.draft.clone().into();

    let actions = Actions::new().change(props.onchange.clone());
    let (form, info) = use_form(&info, actions);

    let RoundInfo { answer, points, guesses } = info;

    let color = {
        let form = form.clone();
        move |name: &str| form.error(name).map(|_| Color::Danger)
    };

    html! {
        <div class="pt-5 pl-4 pr-5">
        <SimpleField label="Answer" help={form.error("answer")} >
            <Input oninput={form.field(|x| &mut x.answer)} value={answer} color={color("answer")}/>
        </SimpleField>

        <SimpleField label="Points" help={form.error("points")} >
            <EnumButtons<PointChoices> onclick={form.field(|x| &mut x.points)} value={points}
            color={Color::Info} />
        </SimpleField>

        <SimpleField label="Guesses" help={form.error("guesses")}>
            <EnumButtons<GuessChoices> onclick={form.field(|x| &mut x.guesses)} value={guesses}
            color={Color::Info} />
        </SimpleField>

        <Block/>
        <Button fullwidth=true onclick={&props.onremove} light=true color={Color::Danger} hidden={!has_image}>
        {"Remove image"}
        </Button>

        </div>
    }
}
