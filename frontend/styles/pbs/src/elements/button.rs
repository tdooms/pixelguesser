use yew::prelude::*;

use crate::properties::{
    Color, Disabled, Fullwidth, Hidden, Inverted, Light, Loading, Outlined, Rounded, Selected, Size,
};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub hidden: Hidden,

    #[prop_or_default]
    pub outlined: Outlined,

    #[prop_or_default]
    pub inverted: Inverted,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub light: Light,

    #[prop_or_default]
    pub loading: Loading,

    #[prop_or_default]
    pub disabled: Disabled,

    #[prop_or_default]
    pub fullwidth: Fullwidth,

    #[prop_or_default]
    pub selected: Selected,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub extra: String,
}

/// A generic button
/// [https://bulma.io/documentation/elements/button/](https://bulma.io/documentation/elements/button/)
#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let classes = classes!(
        "button",
        props.hidden,
        props.outlined,
        props.light,
        props.inverted,
        props.rounded,
        props.loading,
        props.fullwidth,
        props.selected,
        props.color,
        props.size,
        &props.extra
    );

    let onclick = props.onclick.reform(|_| ());

    html! {
        <button class={classes} onclick={onclick}>
            { for props.children.iter() }
        </button>
    }
}
