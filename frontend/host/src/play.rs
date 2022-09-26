use std::collections::HashMap;

use yew::*;

use api::{Action, Player, Resolution, Round, Stage};

use components::Pixelate;
use shared::host::INFO_DURATION;

use crate::info::Info;
use crate::ranking::Ranking;
use gloo::timers::callback::Timeout;
use web_sys::HtmlImageElement;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub round: Round,
    pub index: usize,
    pub rounds: usize,

    pub stage: Stage,
    pub players: HashMap<String, Player>,
    pub callback: Callback<Action>,
}

#[function_component(Play)]
pub fn play(props: &Props) -> Html {
    let Props { round, index, rounds, stage, players, callback } = props.clone();
    let reveal = callback.reform(|_| Action::Stage(Stage::Revealed));

    let timer = use_state(|| Timeout::new(0, || ()));
    let image = use_state(|| None);

    use_effect_with_deps(
        move |_| {
            let cb = move || callback.emit(Action::Stage(Stage::Running));
            timer.set(Timeout::new(1_000 * INFO_DURATION, cb));
            || ()
        },
        index,
    );

    let src = round.image.src(Resolution::HD);
    let image_c = image.clone();
    use_effect_with_deps(
        move |_index| {
            let element = HtmlImageElement::new().unwrap();
            element.set_src(&src);
            image_c.set(Some(element));
            || ()
        },
        index,
    );

    match (stage, (*image).clone()) {
        (Stage::Info, _) => html! {
            <Info {index} {rounds} {round}/>
        },
        (Stage::Running | Stage::Paused | Stage::Revealing | Stage::Revealed, Some(image)) => {
            html! {
                <Pixelate {stage} {image} {reveal}/>
            }
        }
        (Stage::Running | Stage::Paused | Stage::Revealing | Stage::Revealed, None) => html! {
            "image not loaded"
        },
        (Stage::Scores, _) => html! {
            <Ranking {players}/>
        },
    }
}
