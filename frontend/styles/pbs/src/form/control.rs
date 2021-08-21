use crate::properties::Expanded;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: Expanded,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
#[function_component(Control)]
pub fn control(props: &Props) -> Html {
    let classes = classes!("control", &props.extra, props.expanded);

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}
