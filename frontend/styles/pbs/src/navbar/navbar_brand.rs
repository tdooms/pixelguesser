use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

#[function_component(NavbarBrand)]
pub fn navbar_brand(props: &Props) -> Html {
    html! {
        <div class="navbar-brand">
            { for props.children.iter() }
        </div>
    }
}