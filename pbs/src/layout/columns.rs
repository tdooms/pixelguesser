use yew::prelude::*;
use yewtil::NeqAssign;

use crate::classify;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// Align child columns vertically.
    #[prop_or_default]
    pub vcentered: bool,
    /// Allow for multiline rows.
    #[prop_or_default]
    pub multiline: bool,
    /// Center all child columns within their row.
    #[prop_or_default]
    pub centered: bool,
}

/// The container for a set of responsive columns.
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
pub struct Columns {
    props: ColumnsProps,
}

impl Component for Columns {
    type Message = ();
    type Properties = ColumnsProps;

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
        let ColumnsProps { vcentered, multiline, centered, .. } = self.props;
        let classes =
            classes!("columns", &self.props.extra, classify!(vcentered, multiline, centered));
        html! {
            <div class={classes}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
