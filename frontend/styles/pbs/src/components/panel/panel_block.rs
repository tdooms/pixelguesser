use crate::properties::{Active, Color};
use crate::utils::enclose;
use crate::utils::enclose_with_tag;
use yew::prelude::*;

// TODO: this can only be: control, input, button, panel-icon
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub active: Active,
}

#[function_component(PanelBlock)]
pub fn panel_block(props: &Props) -> Html {
    let classes = classes!("panel-block", &props.extra, props.active);
    html! {
        <nav class={classes}>
            { for props.children.iter() }
        </nav>
    }
}
