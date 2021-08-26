use yew::prelude::*;

use crate::properties::{Alignment, Separator, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    /// extra classes
    #[prop_or_default]
    pub extra: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<Separator>,
}

/// A simple breadcrumb component to improve your navigation experience.
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &Props) -> Html {
    let classes =
        classes!("breadcrumb", props.size, props.alignment, props.separator, &props.extra);

    html! {
        <nav class={classes} aria-label="breadcrumbs">
            <ul>
                { for props.children.iter() }
            </ul>
        </nav>
    }
}
