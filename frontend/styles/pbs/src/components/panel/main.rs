use yew::prelude::*;
use crate::utils::enclose;
use crate::properties::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub heading: Option<Html>,

    #[prop_or_default]
    pub color: Option<Color>
}

#[function_component(Panel)]
pub fn panel(props: &Props) -> Html {
    let classes = classes!("panel", &props.extra, props.color);
    html! {
        <nav class={classes}>
            {enclose_with_tag("p", "panel-heading", props.heading)}
            { for props.children.iter() }
        </nav>
    }
}