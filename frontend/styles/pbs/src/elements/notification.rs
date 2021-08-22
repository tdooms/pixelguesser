use yew::prelude::*;

use crate::properties::{Color, Light};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub light: Light,
}

/// Bold notification blocks, to alert your users of something.
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
#[function_component(Notification)]
pub fn notification(props: &Props) -> Html {
    let classes = classes!("notification", &props.extra, props.color, props.light);
    let onclick = props.onclick.reform(|_| ());

    html! {
        <div class={classes}>
            <button class="delete" onclick={onclick}></button>
            { for props.children.iter() }
        </div>
    }
}
