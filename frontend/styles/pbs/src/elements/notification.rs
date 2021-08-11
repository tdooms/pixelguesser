use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::{classify, Color};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct NotificationProps {
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub light: bool,
}

/// Bold notification blocks, to alert your users of something.
///
/// [https://bulma.io/documentation/elements/notification/](https://bulma.io/documentation/elements/notification/)
pub struct Notification {
    props: NotificationProps,
    link: ComponentLink<Self>,
}

impl Component for Notification {
    type Message = ();
    type Properties = NotificationProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.props.onclick.emit(());
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let NotificationProps { light, .. } = self.props;
        let classes = classes!(
            "notification",
            &self.props.extra,
            self.props.color.as_ref().map(ToString::to_string),
            classify!(light)
        );
        html! {
            <div class={classes}>
                <button class="delete" onclick={self.link.callback(|_| ())}></button>
                { for self.props.children.iter() }
            </div>
        }
    }
}
