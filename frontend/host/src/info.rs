use yew::prelude::*;

use api::Round;
use cobul::props::{Color, HeaderSize, HeroSize};
use cobul::*;
use gloo::timers::callback::{Timeout};
use shared::HOST_INFO_DURATION;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,

    pub round: Round,
}

#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let Props { index, rounds, round } = props.clone();

    let countdown = use_state(|| HOST_INFO_DURATION);
    let timer = use_state(|| Timeout::new(0, || ()));
    use_effect_with_deps(
        move |countdown| {
            if **countdown > 0 {
                let cloned = countdown.clone();
                timer.set(Timeout::new(1_000, move || cloned.set(*cloned - 1)));
            };
            || ()
        },
        countdown.clone(),
    );

    let points = match round.points as u64 {
        1 => "1 Point".to_owned(),
        x => format!("{} Points", x),
    };

    let guesses = match round.guesses as u64 {
        1 => "1 Guess".to_owned(),
        x => format!("{} Guesses", x),
    };

    html! {
        <>
        <Hero color={Color::Primary} size={HeroSize::Medium}>
            <Container class="has-text-centered">
                <Title size={HeaderSize::Is1}> {format!("Starting round {}/{}", index + 1, rounds)} </Title>
                <Block>
                    <Title size={HeaderSize::Is4}> {points} </Title>
                    <Title size={HeaderSize::Is4}> {guesses} </Title>
                </Block>
            </Container>
        </Hero>
        <Hero>
        <Container class="has-text-centered">
            <Block/>
            <Title style="font-size:60px" size={HeaderSize::Is1}> {countdown.to_string()} </Title>
        </Container>
        </Hero>
        </>
    }
}
