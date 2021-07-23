use std::collections::HashMap;

use yew::prelude::*;
use yewtil::NeqAssign;

use api::Round;

pub enum Msg {
    Clicked(usize),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<usize>,
    pub images: Vec<String>,
}

pub struct SideImage {
    props: Props,
}

impl Component for SideImage {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let map_view = |src| html! { <pbs::DynImage src={src} height=10/> };

        html! {
            { for self.props.images.iter(map_view) }
        }
    }
}
