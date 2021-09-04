use yew::prelude::*;

use crate::properties::HeaderSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    /// The size of this component.
    #[prop_or_default]
    pub size: Option<HeaderSize>,
}

/// A simple heading to add depth to your page.
///
/// [https://bulma.io/documentation/elements/title/](https://bulma.io/documentation/elements/title/)
#[function_component(Subtitle)]
pub fn subtitle(props: &Props) -> Html {
    let size = props.size.unwrap_or(HeaderSize::Is5);
    let classes = classes!("subtitle", &props.extra, size);

    html! {
        <p class={classes}>
            { for props.children.iter() }
        </p>
    }
}
