use yew::prelude::*;

use cobul::props::HeroSize;
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
    let points = match props.round.points as u64 {
        1 => "1 point".to_owned(),
        x => format!("{} points", x),
    };

    let header = html! {
        <Subtitle> {"round "} {props.index + 1} {" / "} {props.rounds} </Subtitle>
    };

    html! {
        <Hero size={HeroSize::Medium} header={header}>
            <Container extra="has-text-centered">
                <Title> {props.round.answer.clone()} </Title>
                <Subtitle> {points} </Subtitle>
            </Container>
        </Hero>
    }
}
