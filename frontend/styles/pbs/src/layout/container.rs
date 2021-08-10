use yew::prelude::*;
use yewtil::NeqAssign;

use crate::classify;
use crate::common::ContainerSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ContainerProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub size: Option<ContainerSize>,
}

/// A simple container to center your content horizontally.
///
/// [https://bulma.io/documentation/layout/container/](https://bulma.io/documentation/layout/container/)
pub struct Container {
    props: ContainerProps,
}

impl Component for Container {
    type Message = ();
    type Properties = ContainerProps;

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
            "container",
            &self.props.extra,
            self.props.size.as_ref().map(ToString::to_string)
        );

        html! {
            <div class={classes}>
                { for self.props.children.iter() }
            </div>
        }
    }
}
