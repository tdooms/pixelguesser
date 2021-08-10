use yew::prelude::*;
use yewtil::NeqAssign;

use crate::utils::bytes_to_url;
use pbs::ColumnSize;

pub enum Msg {
    Clicked(usize),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<usize>,
    pub images: Vec<Option<Vec<u8>>>,
    pub current: usize,
}

pub struct SideImages {
    link: ComponentLink<Self>,
    props: Props,
}

impl Component for SideImages {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(index) => self.props.onclick.emit(index),
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let map_view = |(index, src): (usize, &Option<Vec<u8>>)| {
            let image = match src {
                Some(src) => html! { <cbs::DynImage src={bytes_to_url(src)} height=10/> },
                None => html! {},
            };
            let grey = (index == self.props.current)
                .then(|| "has-background-white-ter")
                .unwrap_or_default();

            html! {
                <div class=classes!("columns", grey) onclick={self.link.callback(move |_| Msg::Clicked(index))}>
                    <pbs::Column size={ColumnSize::IsNarrow}> <p> {index} </p> </pbs::Column>
                    <pbs::Column extra="p-1"> {image} </pbs::Column>
                </div>
            }
        };

        html! {
            <div class="p-4" style="overflow-y:auto">
                { for self.props.images.iter().enumerate().map(map_view) }
            </div>
        }
    }
}
