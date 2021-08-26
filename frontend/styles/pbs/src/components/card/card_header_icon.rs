use yew::prelude::*;

// https://bulma.io/documentation/components/card/
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    children: Children,
}

#[function_component(CardHeaderIcon)]
pub fn card_header_icon(props: &Props) -> Html {
    html! {
        <nav class={"card-header-icon"} >
            { for props.children.iter() }
        </nav>
    }
}