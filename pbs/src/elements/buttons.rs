use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

/// A white box to contain other elements.
///
/// [https://bulma.io/documentation/elements/button/#list-of-buttons](https://bulma.io/documentation/elements/button/#list-of-buttons)
pub struct Buttons {
    props: ButtonsProps,
}

impl Component for Buttons {
    type Message = ();
    type Properties = ButtonsProps;

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
        html! {
            <div class=classes!("buttons", &self.props.extra)>
                { for self.props.children.iter() }
            </div>
        }
    }
}
