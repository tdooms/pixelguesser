use yew::prelude::*;

use crate::properties::{Mobile, TextCentered};
use crate::utils::enclose;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub left: Option<Html>,

    #[prop_or_default]
    pub right: Option<Html>,

    #[prop_or_default]
    pub centered: TextCentered,

    #[prop_or_default]
    pub mobile: Mobile,
}

#[function_component(Level)]
pub fn level(props: &Props) -> Html {
    let classes = classes!("level", &props.extra, props.centered, props.mobile);
    html! {
        <div class={classes}>
            { enclose("media-left", props.left.clone()) }
            { for props.children.iter() }
            { enclose("media-right", props.right.clone()) }
        </div>
    }
}