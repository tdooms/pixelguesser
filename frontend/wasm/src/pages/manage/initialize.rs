use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::{Color, InputType, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onchange: Callback<String>,
}

pub enum Msg {
    Submit,
    Value(String),
}

pub struct Initialize {
    value: String,
}

impl Component for Initialize {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {value: String::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                ctx.props().onchange.emit(std::mem::take(&mut self.value));
            }
            Msg::Value(value) => self.value = value,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(Msg::Value);
        let onclick = ctx.link().callback(|_| Msg::Submit);

        html! {
            <Field grouped=true>
                <Control expanded=true>
                    <Input size={Size::Large} r#type={InputType::Text} placeholder={"eg. Alex"} value={self.value.clone()} oninput={oninput}/>
                </Control>
                <Control>
                    <Button size={Size::Large} color={Color::Info} onclick={onclick}> <Icon icon="fas fa-plus"> </Icon> </Button>
                </Control>
            </Field>
        }
    }
}
