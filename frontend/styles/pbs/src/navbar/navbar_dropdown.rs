use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

#[function_component(NavbarDropdown)]
pub fn navbar_dropdown(props: &Props) -> Html {
    html! {
        <div class="navbar-dropdown">
            { for props.children.iter() }
        </div>
    }
}