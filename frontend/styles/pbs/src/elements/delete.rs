#![allow(clippy::redundant_closure_call)]

use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct DeleteProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,

    pub onclick: Callback<MouseEvent>,
}

/// A versatile delete cross.
///
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
pub struct Delete {
    props: DeleteProps,
}

impl Component for Delete {
    type Message = ();
    type Properties = DeleteProps;

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
        let classes = classes!("delete", &self.props.extra);
        html! {
            <@{self.props.tag.clone()} class={classes} onclick={self.props.onclick.clone()}>
                { for self.props.children.iter() }
            </@>
        }
    }
}
