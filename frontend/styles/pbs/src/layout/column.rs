use yew::prelude::*;

use crate::properties::{ColumnOffset, ColumnSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub size: Option<ColumnSize>,

    #[prop_or_default]
    pub offset: Option<ColumnOffset>,
}

/// A flexbox-based responsive column.
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
#[function_component(Column)]
pub fn column(props: &Props) -> Html {
    let classes = classes!("column", &props.extra, props.size, props.offset);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
