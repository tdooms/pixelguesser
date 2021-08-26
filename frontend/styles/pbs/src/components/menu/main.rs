use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/box/](https://bulma.io/documentation/elements/box/)
#[function_component(Menu)]
pub fn menu(props: &Props) -> Html {
    let classes = classes!("menu", &props.extra);
    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}