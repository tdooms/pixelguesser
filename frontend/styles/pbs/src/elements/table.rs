use yew::prelude::*;

use crate::properties::{Bordered, Fullwidth, Hoverable, Narrow, Striped};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: Bordered,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: Striped,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: Narrow,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: Hoverable,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: Fullwidth,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
#[function_component(Table)]
pub fn table(props: &Props) -> html {
    let classes = classes!(
        "table",
        &props.extra,
        props.bordered,
        props.striped,
        props.narrow,
        props.hoverable,
        props.fullwidth,
    );

    let table = html! {
        <table class={classes}>
            { for props.children.iter() }
        </table>
    };

    match props.scrollable {
        true => html! {<div class="table-container"> {table} </div> },
        false => table,
    }
}
