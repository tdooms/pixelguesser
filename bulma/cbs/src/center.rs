use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct CenterProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct Center {
    props: CenterProps,
}

impl Component for Center {
    type Message = ();
    type Properties = CenterProps;

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
            <div class="columns is-centered is-desktop is-vcentered" style="height:100vh">
                <div class="column">
                    { for self.props.children.iter() }
                </div>
            </div>
        }
    }
}
