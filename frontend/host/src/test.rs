use yew::*;
use api::{Guesses, Points, Round};

use crate::info::Info;

#[function_component(Test)]
pub fn test() -> Html {
    let round = Round {
        quiz_id: None,
        round_index: 0,
        answer: "Paris".to_string(),
        points: Points::One,
        guesses: Guesses::Two,
        speed: 0,
        algorithm: Default::default(),
        image: Default::default(),
    };

    html! {
        <Info rounds=20 index=3 {round} />
    }
}