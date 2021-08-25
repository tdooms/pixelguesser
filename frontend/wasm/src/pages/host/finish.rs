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
    let body = html! { <Scores players={players.clone()}/> };

    html! {
        <>
            <cbs::TitleHero color={Color::Primary} title={quiz.name.clone()} subtitle={"finished"}/>
            <Hero body={body} color={Color::Primary} size={HeroSize::Medium}/>
        </>
    }
}