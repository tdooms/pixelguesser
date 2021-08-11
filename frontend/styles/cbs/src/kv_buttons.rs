use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::{Alignment, Color};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct KvButtonsProps<T: Clone + PartialEq + 'static> {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub alignment: Alignment,

    #[prop_or_default]
    pub color: Option<Color>,

    pub values: Vec<(String, T)>,
}

pub struct KvButtons<T: Clone + PartialEq + 'static> {
    props: KvButtonsProps<T>,
    link: ComponentLink<Self>,
    selected: Option<usize>,
}

impl<T: Clone + PartialEq + 'static> Component for KvButtons<T> {
    type Message = usize;
    type Properties = KvButtonsProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let selected = (!props.values.is_empty()).then(|| 0);
        Self { props, link, selected }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.selected = Some(msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let button_map = |index: usize, key: String| {
            let selected = Some(index) == self.selected;
            let color = selected.then(|| self.props.color).flatten();
            let onclick = self.link.callback(move |_| index);

            html! {
                <pbs::Button color={color} onclick={onclick} selected={selected}>
                    {key}
                </pbs::Button>
            }
        };

        html! {
            <pbs::Buttons addons=true alignment={self.props.alignment}>
                { for self.props.values.iter().enumerate().map(|(index, (key, _))| button_map(index, key.clone())) }
            </pbs::Buttons>
        }
    }
}
