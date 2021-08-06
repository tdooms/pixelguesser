use yew::prelude::*;

use pbs::{Color, InputType, Size};

#[derive(Clone, Properties)]
pub struct Props {
    pub onchange: Callback<String>,
}

pub enum Msg {
    Submit,
    Value(String),
}

pub struct Initialize {
    link: ComponentLink<Self>,
    props: Props,
    value: String,
}

impl Component for Initialize {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, value: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                self.props.onchange.emit(std::mem::take(&mut self.value));
            }
            Msg::Value(value) => self.value = value,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let oninput = self.link.callback(Msg::Value);
        let onclick = self.link.callback(|_| Msg::Submit);

        let input = html! {};

        let button = html! {};

        html! {
            <pbs::Field grouped=true>
                <pbs::Control expanded=true>
                    <pbs::Input size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} value={self.value.clone()} oninput={oninput}/>
                </pbs::Control>
                <pbs::Control>
                    <cbs::IconButton size={Size::Large} color={Color::Info} onclick={onclick} icon="fas fa-plus"/>
                </pbs::Control>
            </pbs::Field>
        }
    }
}
