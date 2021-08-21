use yew::prelude::*;

use crate::properties::{Alignment, Boxed, Fullwidth, Rounded, Size, Toggle};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// Add a more classic style with borders to this component.
    #[prop_or_default]
    pub boxed: Boxed,
    /// Add the "radio button" style to the elements of this component.
    #[prop_or_default]
    pub toggle: Toggle,
    /// Make the tab elements of this component rounded.
    #[prop_or_default]
    pub rounded: Rounded,
    /// Make this component fullwidth.
    #[prop_or_default]
    pub fullwidth: Fullwidth,
}

/// Simple responsive horizontal navigation tabs, with different styles.
///
/// [https://bulma.io/documentation/components/tabs/](https://bulma.io/documentation/components/tabs/)
///
/// For integration with Yew Router, it is recommended that the `RouterButton` or `RouterAnchor`
/// components be used as the individual tab elements for this component.
#[function_component(Tabs)]
pub fn tabs(props: &Props) -> Html {
    let classes = classes!(
        "tabs",
        &props.extra,
        props.size,
        props.boxed,
        props.toggle,
        props.rounded,
        props.fullwidth
    );

    html! {
        <div class={classes}>
            <ul>
                { for props.children.iter() }
            </ul>
        </div>
    }
}
