use yew::prelude::*;
use crate::properties::{Dropdown, Hoverable};

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children,
    dropdown: Dropdown,
    hoverable: Hoverable
}

#[function_component(NavbarItem)]
pub fn navbar_item(props: &Props) -> Html {
    let tag = if props.dropdown { "a" } else { "div" };

    html! {
        <@tag class=classes!("navbar-item", props.dropdown, props.hoverable)>
            { for props.children.iter() }
        </@tag>
    }
}