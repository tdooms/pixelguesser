use yew::prelude::*;

use crate::properties::{Centered, Gapless, Multiline, VCentered};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    /// Align child columns vertically.
    #[prop_or_default]
    pub vcentered: VCentered,

    /// Allow for multiline rows.
    #[prop_or_default]
    pub multiline: Multiline,

    /// Center all child columns within their row.
    #[prop_or_default]
    pub centered: Centered,

    /// Remove the gaps between columns.
    #[prop_or_default]
    pub gapless: Gapless,
}

/// The container for a set of responsive columns.
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
#[function_component(Columns)]
pub fn columns(props: &Props) -> Html {
    let classes = classes!(
        "columns",
        &props.extra,
        props.vcentered,
        props.multiline,
        props.centered,
        props.gapless
    );

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
