use yew::prelude::*;

// https://bulma.io/documentation/components/card/
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    children: Children,
}

#[function_component(CardFooterItem)]
pub fn card(props: &Props) -> Html {
    html! {
        <nav class={"card-footer-item"} >
            { for props.children.iter() }
        </nav>
    }
}