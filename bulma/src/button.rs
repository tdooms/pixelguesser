use crate::color::Color;
use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub onclick: Callback<MouseEvent>,

    #[prop_or_default]
    pub hidden: bool,

    #[prop_or_default]
    pub large: bool,

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
    pub color: Color,

    #[prop_or_default]
    pub icon: Option<String>,

    #[prop_or_default]
    pub text: Option<String>,

    #[prop_or_default]
    pub extra: Classes,
}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
}

impl Button {
    fn classes(&self) -> Classes {
        let hidden = self.props.hidden.then(|| "is-hidden");
        let large = self.props.hidden.then(|| "is-large");
        let outlined = self.props.hidden.then(|| "is-outlined");
        let light = self.props.hidden.then(|| "is-light");
        let inverted = self.props.hidden.then(|| "is-inverted");
        let rounded = self.props.hidden.then(|| "is-rounded");
        let loading = self.props.hidden.then(|| "is-loading");

        let color = self.props.color.to_string();

        classes!("button", hidden, large, outlined, light, inverted, rounded, loading, color, &self.props.extra)
    }

    fn view_icon(&self) -> Html {
        match &self.props.icon {
            Some(icon) => html! { <span class="icon"><i class={icon}></i></span> }
            None => html!{}
        }
    }
    fn view_text(&self) -> Html {
        match &self.props.text {
            Some(text) => html! { <span>{text}</span> }
            None => html!{}
        }
    }
}

impl Component for Button {
    type Message = MouseEvent;
    type Properties = Props;

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
        html! {
            <button class=self.classes() onclick=self.link.callback(|e| e)>
                { self.view_icon() }
                { self.view_text() }
            </button>
        }
    }
}
