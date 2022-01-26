use yew::prelude::*;

use cobul::props::{Color, HeroSize};
use cobul::*;
use sessions::Player;

use super::Ranking;
use crate::graphql::Quiz;
use std::collections::HashMap;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub players: HashMap<String, Player>,
    pub quiz: Quiz,
}

#[function_component(Finish)]
pub fn finish(props: &Props) -> Html {
    let Props { players, quiz, .. } = &props;
    let _body = html! {};

    html! {
        <>
            <Hero color={Color::Primary}>
                <Title> {quiz.title.clone()} </Title>
                <Subtitle> {"finished"} </Subtitle>
            </Hero>

            <Hero color={Color::Primary} size={HeroSize::Medium}>
                <Ranking players={players.clone()}/>
            </Hero>
        </>
    }
}
