use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HelpProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

pub struct Help {
    props: HelpProps,
}

impl Component for Help {
    type Message = ();
    type Properties = HelpProps;

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
            <div class=classes!("help", &self.props.extra)>
                { for self.props.children.iter() }
            </div>
        }
    }
}
