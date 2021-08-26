use yew::prelude::*;

use crate::properties::{Addons, Grouped, GroupedMultiline};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub grouped: Grouped,

    #[prop_or_default]
    pub multiline: GroupedMultiline,

    #[prop_or_default]
    pub addons: Addons,
}

#[function_component(Field)]
pub fn field(props: &Props) -> Html {
    let classes = classes!("field", &props.extra, props.multiline, props.addons, props.grouped,);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
