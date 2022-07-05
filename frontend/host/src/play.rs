use std::collections::HashMap;

use yew::*;

use api::{Action, Player, Round, Stage};

use components::Pixelate;
use shared::HOST_INFO_DURATION;

use crate::info::Info;
use crate::ranking::Ranking;
use gloo::timers::callback::Timeout;

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
    let onreveal = callback.reform(|_| Action::Stage(Stage::Revealed));
    let timer = use_state(|| Timeout::new(0, || ()));

    use_effect_with_deps(
        move |_| {
            let cb = move || callback.emit(Action::Stage(Stage::Running));
            timer.set(Timeout::new(1_000 * HOST_INFO_DURATION, cb));
            || ()
        },
        index,
    );

    match stage {
        Stage::Info => html! {
            <Info {index} {rounds} {round}/>
        },
        Stage::Running | Stage::Paused | Stage::Revealing | Stage::Revealed => html! {
            <Pixelate {stage} image={round.image} {onreveal}/>
        },
        Stage::Scores => html! {
            <Ranking {players}/>
        },
    }
}
