use yew::prelude::*;
use crate::properties::{Transparent, Color, Spaced, Shadow};

// TODO: is fixed navbar possible without too much hassle?
// https://bulma.io/documentation/components/navbar/#fixed-navbar
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    children: Children,
    transparent: Transparent,
    spaced: Spaced,
    shadow: Shadow,
    color: Option<Color>,
}

#[function_component(Navbar)]
pub fn navbar(props: &Props) -> Html {
    let classes = classes!("navbar", props.transparent, props.color, props.spaced, props.shadow);
    html! {
        <nav class={classes} role="navigation" aria-label="main navigation">
            { for props.children.iter() }
        </nav>
    }
}