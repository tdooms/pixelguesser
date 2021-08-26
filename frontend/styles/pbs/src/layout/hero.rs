use yew::prelude::*;

use crate::properties::{Color, HeroSize};
use crate::utils::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<HeroSize>,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub footer: Option<Html>,
}

/// An imposing hero banner to showcase something.
///
/// [https://bulma.io/documentation/layout/hero/](https://bulma.io/documentation/layout/hero/)
#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    let classes = classes!("hero", props.size, props.color, &props.extra);

    html! {
        <section class={classes}>
            { enclose("hero-header", props.header.clone()) }
            <div class="hero-body"> { for props.children.iter() } </div>
            { enclose("hero-footer", props.footer.clone()) }
        </section>
    }
}
