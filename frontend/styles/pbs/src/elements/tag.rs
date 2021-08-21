use yew::prelude::*;

use crate::properties::{Delete, Rounded, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "span".into())]
    pub tag: String,
    /// The click handler for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// Make this tag rounded.
    #[prop_or_default]
    pub rounded: Rounded,
    /// Turn this tag into a delete button.
    #[prop_or_default]
    pub delete: Delete,
    /// The size for this component.
    #[prop_or_default]
    pub size: Size,
}

/// A small tag label to insert anywhere.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tag)]
pub fn tag(props: &Props) -> Html {
    let classes = classes!("tag", &props.extra, props.size, props.rounded, props.delete);

    html! {
        <@{ props.tag.clone() } class={classes} onclick={props.onclick.clone()}>
            { for props.children.iter() }
        </@>
    }
}
