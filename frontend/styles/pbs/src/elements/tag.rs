use yew::prelude::*;

use crate::properties::{Delete, Light, Rounded, Size};

// The turn into delete button is intentionally skipped
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub light: Light,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    /// The click handler for this component.
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    /// Make this tag rounded.
    #[prop_or_default]
    pub rounded: Rounded,

    /// The size for this component.
    #[prop_or_default]
    pub size: Size,
}

/// A small tag label to insert anywhere.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let classes = classes!("tag", &props.extra, props.size, props.rounded, props.light,);

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}
