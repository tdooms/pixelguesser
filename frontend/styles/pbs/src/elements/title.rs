use yew::prelude::*;

use crate::properties::{HeaderSize, Spaced};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// Maintain the normal spacing between titles and subtitles.
    #[prop_or_default]
    pub spaced: Spaced,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Title)]
pub fn title(props: &Props) -> Html {
    let classes = classes!("title", &props.extra, props.size, props.spaced);

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}

//////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SubtitleProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "h3".into())]
    pub tag: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Subtitle)]
pub fn subtitle(props: &Props) -> Html {
    let classes = classes!("subtitle", &props.extra, props.size);

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}
