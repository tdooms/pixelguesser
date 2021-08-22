use yew::prelude::*;

use pbs::properties::{Color, HeroSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,
    /// The size for this hero.
    #[prop_or_default]
    pub size: Option<HeroSize>,

    pub title: String,

    pub subtitle: String,
}

#[function_component(TitleHero)]
pub fn title_hero(props: &Props) -> Html {
    let classes = classes!("hero", props.size, props.color, &props.extra);

    html! {
        <section class={classes}>
            <div class="hero-body">
                <p class="title"> {&props.title} </p>
                <p class="subtitle"> {&props.subtitle} </p>
            </div>
        </section>
    }
}
