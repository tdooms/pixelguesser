use yew::prelude::*;

use cobul::props::{Color, HeaderSize, HeroSize};
use cobul::*;

use crate::graphql::Round;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,

    pub round: Round,
}

#[function_component(RoundInfo)]
pub fn round_info(props: &Props) -> Html {
    let Props { index, rounds, round } = props.clone();

    let points = match round.points as u64 {
        1 => "1 Point".to_owned(),
        x => format!("{} Points", x),
    };

    let guesses = match round.guesses as u64 {
        1 => "1 Guess".to_owned(),
        x => format!("{} Guesses", x),
    };

    html! {
        <Hero color={Color::Primary} size={HeroSize::Medium}>
            <Container class="has-text-centered">
                <Title size={HeaderSize::Is1}> {format!("Starting round {}/{}", index + 1, rounds)} </Title>
                <Block>
                    <Title size={HeaderSize::Is4}> {points} </Title>
                    <Title size={HeaderSize::Is4}> {guesses} </Title>
                </Block>
            </Container>
        </Hero>
    }
}
