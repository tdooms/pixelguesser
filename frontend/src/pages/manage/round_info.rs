use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::HeroSize;

use crate::graphql::Round;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,
    pub round: Round,
}

#[function_component(RoundInfo)]
pub fn round_info(props: &Props) -> Html {
    let header = html! {
        <Subtitle> {"round "} {props.index} {" / "} {props.rounds} </Subtitle>
    };

    html! {
        <Hero size={HeroSize::Medium} header={header}>
            <Container extra="has-text-centered">
                <Title> {props.round.info.answer.clone()} </Title>
                <Subtitle> {props.round.info.points} {" points"} </Subtitle>
            </Container>
        </Hero>
    }
}
