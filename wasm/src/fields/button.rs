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
    pub light: bool,

    #[prop_or_default]
    pub color: String,

    #[prop_or_default]
    pub icon: String,

    #[prop_or_default]
    pub text: String,

    #[prop_or_default]
    pub extra: Classes,
}

pub struct Button {
    link: ComponentLink<Self>,
    props: Props,
}

impl Button {
    fn classes(&self) -> Classes {
        let mut vec = vec![];

        if self.props.hidden {
            vec.push("is-hidden")
        }
        if self.props.large {
            vec.push("is-large")
        }
        if self.props.outlined {
            vec.push("is-outlined")
        }
        if self.props.light {
            vec.push("is-light")
        }

        classes!("button", vec, &self.props.color, self.props.extra.clone())
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
                <span class="icon"><i class={&self.props.icon}></i></span> <span>{&self.props.text}</span>
            </button>
        }
    }
}
