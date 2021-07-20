use yew::events::MouseEvent;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::common::TextColor;
use crate::{Alignment, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct IconProps {
    #[prop_or_default]
    pub text: Option<String>,

    pub icon: String,

    #[prop_or_default]
    pub extra: String,

    /// The click handler to use for this component.
    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
    /// The size of this component; to help prevent page "jumps" during load.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<TextColor>,
}

/// A container for any type of icon font.
///
/// [https://bulma.io/documentation/elements/icon/](https://bulma.io/documentation/elements/icon/)
pub struct Icon {
    props: IconProps,
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

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
            "icon",
            &self.props.extra,
            self.props.size.to_string(),
            self.props.color.as_ref().map(ToString::to_string)
        );

        let icon = html! {
            <span class=classes onclick=self.props.onclick.clone()>
                <i class=self.props.icon.clone()> </i>
            </span>
        };

        match &self.props.text {
            Some(text) => html! {<span class="icon-text"> { icon } <span>{ text }</span> </span>},
            None => icon,
        }
    }
}
