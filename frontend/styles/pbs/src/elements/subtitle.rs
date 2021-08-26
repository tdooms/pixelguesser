use yew::prelude::*;

use crate::properties::HeaderSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
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
