use yew::prelude::*;

use crate::properties::{Addons, Alignment};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub addons: Addons,
}

/// TODO
/// [https://bulma.io/documentation/elements/button/#list-of-buttons](https://bulma.io/documentation/elements/button/#list-of-buttons)
#[function_component(Buttons)]
pub fn buttons(props: &Props) -> Html {
    let classes = classes!("buttons", &props.extra, props.alignment, props.addons);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
