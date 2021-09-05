use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::{Color, HeroSize};
use shared::Player;

use crate::graphql::Quiz;
use crate::pages::host::Scores;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub players: Vec<Player>,
    pub quiz: Quiz,
}

#[function_component(Finish)]
pub fn finish(props: &Props) -> Html {
    let Props { players, quiz, .. } = &props;
    let body = html! {  };

    html! {
        <>
            <cbs::TitleHero color={Color::Primary} title={quiz.name.clone()} subtitle={"finished"}/>
            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Scores players={players.clone()}/>
            </Hero>
        </>
    }
}