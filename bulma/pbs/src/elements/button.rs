use yew::prelude::*;

use crate::classify;
use crate::{Color, Size};

#[derive(Properties, Clone)]
pub struct ButtonProps {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub hidden: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub outlined: bool,

    #[prop_or_default]
    pub inverted: bool,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub light: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub fullwidth: bool,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub selected: bool,

    #[prop_or_default]
    pub extra: String,
}

pub struct Button {
    link: ComponentLink<Self>,
    props: ButtonProps,
}

impl Component for Button {
    type Message = MouseEvent;
    type Properties = ButtonProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        self.props.onclick.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let ButtonProps {
            hidden,
            outlined,
            light,
            inverted,
            rounded,
            loading,
            fullwidth,
            selected,
            ..
        } = self.props;

        let classes = classes!(
            "button",
            classify!(hidden, outlined, light, inverted, rounded, loading, fullwidth, selected),
            self.props.color.as_ref().map(ToString::to_string),
            self.props.size.to_string(),
            &self.props.extra
        );

        html! {
            <button class={classes} onclick={self.link.callback(|e| e)}>
            { for self.props.children.iter() }
            </button>
        }
    }
}
