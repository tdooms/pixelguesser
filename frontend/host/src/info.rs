use cobul::*;
use gloo::timers::callback::Timeout;
use yew::*;

use api::{Guesses, Round};
use shared::host::INFO_DURATION;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub rounds: usize,

    pub round: Round,
}

#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let Props { index, rounds, round } = props.clone();

    let countdown = use_state(|| INFO_DURATION);
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

    let guesses = match round.guesses {
        Guesses::One => "1 Guess".to_owned(),
        x => format!("{x} Guesses"),
    };

    html! {
        <>
        <Hero color={Color::Info} size={HeroSize::Medium} class="has-text-centered">
            <Title size={HeaderSize::Is1}> {format!("Round {}/{}", index + 1, rounds)} </Title>
        </Hero>
        <Hero color={Color::Info} size={HeroSize::Small}>
            <Level class="">
                <Title size={HeaderSize::Is3}> {points} </Title>
                <Title size={HeaderSize::Is3}> {guesses} </Title>
            </Level>
        </Hero>
        <Hero size={HeroSize::Medium} class="has-text-centered">
            <Title style="font-size:60px" size={HeaderSize::Is1}> {countdown.to_string()} </Title>
        </Hero>
        </>
    }
}
