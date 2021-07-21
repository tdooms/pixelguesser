use crate::{ColumnOffset, ColumnSize};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ColumnProps {
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
///
/// [https://bulma.io/documentation/columns/](https://bulma.io/documentation/columns/)
///
/// This component has a very large number of valid class combinations which users may want.
/// Modelling all of these is particularly for this component, so for now you are encouraged to
/// add classes to this Component manually via the `classes` prop.
pub struct Column {
    props: ColumnProps,
}

impl Component for Column {
    type Message = ();
    type Properties = ColumnProps;

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
        let classes = classes!(
            "column",
            &self.props.extra,
            self.props.size.as_ref().map(ToString::to_string),
            self.props.offset.as_ref().map(ToString::to_string)
        );

        html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        }
    }
}
