use yew::prelude::*;
use yewtil::NeqAssign;

use crate::classify;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// Add borders to all the cells.
    #[prop_or_default]
    pub bordered: bool,
    /// Add stripes to the table.
    #[prop_or_default]
    pub striped: bool,
    /// Make the cells narrower.
    #[prop_or_default]
    pub narrow: bool,
    /// Add a hover effect on each row.
    #[prop_or_default]
    pub hoverable: bool,
    /// Make the table fullwidth.
    #[prop_or_default]
    pub fullwidth: bool,
    /// Make the table scrollable, wrapping the table in a `div.table-container`.
    #[prop_or_default]
    pub scrollable: bool,
}

/// An HTML table component.
///
/// [https://bulma.io/documentation/elements/table/](https://bulma.io/documentation/elements/table/)
pub struct Table {
    props: TableProps,
}

impl Component for Table {
    type Message = ();
    type Properties = TableProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let TableProps { bordered, striped, narrow, hoverable, fullwidth, scrollable, .. } =
            self.props;

        let classes = classes!(
            "table",
            &self.props.extra,
            classify!(bordered, striped, narrow, hoverable, fullwidth, scrollable)
        );

        let table = html! {
            <table class={classes}>
                { for self.props.children.iter() }
            </table>
        };

        match self.props.scrollable {
            true => html! {<div class="table-container"> {table} </div> },
            false => table,
        }
    }
}
