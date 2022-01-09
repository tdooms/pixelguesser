use yew::prelude::*;

use cobul::props::{Alignment, Color};
use cobul::*;

use crate::graphql::{DraftRound, GuessChoices, PointChoices};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<DraftRound>,
    pub info: DraftRound,
}

#[function_component(SideInfo)]
pub fn side_info(props: &Props) -> Html {
    let cloned = props.info.clone();
    let changer = move |answer, guesses, points| -> DraftRound {
        let mut new = cloned.clone();
        if let Some(x) = answer {
            new.answer = x
        }
        if let Some(x) = guesses {
            new.guesses = x
        }
        if let Some(x) = points {
            new.points = x
        }
        new
    };

    let (c1, c2, c3) = (changer.clone(), changer.clone(), changer.clone());
    let answer = props.onchange.reform(move |answer| c1(Some(answer), None, None));
    let points = props.onchange.reform(move |points| c2(None, None, Some(points)));
    let guesses = props.onchange.reform(move |guesses| c3(None, Some(guesses), None));

    html! {
        <div class="p-6">
            <SimpleField label="Answer">
                <Input oninput={answer} />
            </SimpleField>
            <SimpleField label="Points">
                <EnumButtons<PointChoices> value={props.info.points} color={Color::Link} alignment={Alignment::Centered} onclick={points} />
            </SimpleField>
            <SimpleField label="Guesses">
                <EnumButtons<GuessChoices> value={props.info.guesses} color={Color::Link} alignment={Alignment::Centered} onclick={guesses} />
            </SimpleField>
        </div>
    }
}
