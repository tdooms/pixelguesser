use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
struct Props {
    children: Children
}

#[function_component(NavbarEnd)]
pub fn navbar_end(props: &Props) -> Html {
    html! {
        <div class="navbar-end">
            { for props.children.iter() }
        </div>
    }
}