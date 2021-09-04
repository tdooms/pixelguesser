use yew::prelude::*;

use crate::properties::{HeaderSize, Spaced};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
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
    let size = props.size.unwrap_or(HeaderSize::Is3);
    let classes = classes!("title", &props.extra, size, props.spaced);

    html! {
        <p class={classes}>
            { for props.children.iter() }
        </p>
    }
}
