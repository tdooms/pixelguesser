use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    children: Children
}

#[function_component(NavbarStart)]
pub fn navbar_start(props: &Props) -> Html {
    html! {
        <div class="navbar-start">
            { for props.children.iter() }
        </div>
    }
}