use crate::properties::ContainerSize;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub size: Option<ContainerSize>,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
#[function_component(Container)]
pub fn container(props: &Props) -> Html {
    let classes = classes!("container", &props.extra, props.size);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
