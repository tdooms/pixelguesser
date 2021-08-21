use yew::prelude::*;

use crate::properties::{Color, HeroSize};

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
    pub body: Option<Html>,

    #[prop_or_default]
    pub footer: Option<Html>,
}

/// An imposing hero banner to showcase something.
///
/// [https://bulma.io/documentation/layout/hero/](https://bulma.io/documentation/layout/hero/)
#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    let classes = classes!("hero", props.size, props.color, &props.extra);

    // TODO: fix code duplication
    let header = match &props.header {
        Some(html) => html! { <div class="hero-header"> {html.clone()} </div> },
        None => html! {},
    };

    let body = match &props.body {
        Some(html) => html! { <div class="hero-body"> {html.clone()} </div> },
        None => html! {},
    };

    let footer = match &props.footer {
        Some(html) => html! { <div class="hero-footer"> {html.clone()} </div> },
        None => html! {},
    };

    html! {
        <section class={classes}>
            {header} {body} {footer}
        </section>
    }
}
