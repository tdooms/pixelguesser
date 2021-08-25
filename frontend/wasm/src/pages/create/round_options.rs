use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::prelude::*;
use pbs::properties::{Alignment, Color};

use crate::graphql::{DraftRound, GuessChoices, PointChoices};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<DraftRound>,
    pub draft: DraftRound,
}

#[function_component(RoundOptions)]
pub fn round_options(props: &Props) -> Html {
    let mut cloned = props.draft.clone();
    let answer = props.onchange.reform(move |answer| {
        cloned.answer = answer;
        cloned
    });

    let mut cloned = props.draft.clone();
    let points = props.onchange.reform(move |points| {
        cloned.points = points;
        cloned
    });

    let mut cloned = props.draft.clone();
    let guesses = props.onchange.reform(move |guesses| {
        cloned.guesses = guesses;
        cloned
    });

    html! {
        <div class="p-6">
            <cbs::SimpleField label="Answer">
                <Input oninput={answer} />
            </cbs::SimpleField>
            <cbs::SimpleField label="Points">
                <cbs::KvButtons<PointChoices> value={props.draft.points} color={Color::Link} alignment={Alignment::Centered} onclick={points} />
            </cbs::SimpleField>
            <cbs::SimpleField label="Guesses">
                <cbs::KvButtons<GuessChoices> value={props.draft.guesses} color={Color::Link} alignment={Alignment::Centered} onclick={guesses} />
            </cbs::SimpleField>
        </div>
    }
}