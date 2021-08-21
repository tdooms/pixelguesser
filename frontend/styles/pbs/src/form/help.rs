use yew::prelude::*;

use crate::properties::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,
}

#[function_component(Help)]
pub fn help(props: &Props) -> Html {
    let classes = classes!("help", &props.extra, props.color);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
