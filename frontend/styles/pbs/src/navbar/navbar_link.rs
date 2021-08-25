use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

#[function_component(NavbarLink)]
pub fn navbar_link(props: &Props) -> Html {
    html! {
        <div class="navbar-link">
            { for props.children.iter() }
        </div>
    }
}