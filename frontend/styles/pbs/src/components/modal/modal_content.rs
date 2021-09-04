use yew::prelude::*;

use crate::properties::Active;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub active: Active,
}

#[function_component(ModalContent)]
pub fn modal_content(props: &Props) -> Html {
    let classes = classes!("modal", &props.extra, props.active);

    html! {
        <div class={classes}>
            <div class="modal-background"></div>
            <div class="modal-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
