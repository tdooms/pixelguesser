use crate::{classify, Icon};
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ControlProps {
    #[prop_or_default]
    pub inner: Html,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
    /// A modifier to have the controlled element fill up the remaining space.
    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub left: Option<VChild<Icon>>,

    #[prop_or_default]
    pub right: Option<VChild<Icon>>,
}

/// A container with which you can wrap the form controls.
///
/// [https://bulma.io/documentation/form/general/](https://bulma.io/documentation/form/general/)
pub struct Control {
    props: ControlProps,
}

impl Component for Control {
    type Message = ();
    type Properties = ControlProps;

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
        let ControlProps { expanded, .. } = self.props;
        let classes = classes!(
            "control",
            &self.props.extra,
            classify!(expanded),
            self.props.right.as_ref().map(|_| "has-icons-right"),
            self.props.left.as_ref().map(|_| "has-icons-left")
        );

        html! {
            <@{ self.props.tag.clone() } class=classes>
                { self.props.inner.clone() }
                { self.props.left.clone().map(Html::from).unwrap_or(html!{}) }
                { self.props.right.clone().map(Html::from).unwrap_or(html!{}) }
            </@>
        }
    }
}
