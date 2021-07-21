use crate::agents::WebSocketAgent;
use api::{Post, Request};
use pbs::{Color, InputType, Size};

use yew::agent::Dispatcher;
use yew::prelude::*;

#[derive(Clone, Properties)]
pub struct Props {
    pub session_id: u64,
}

pub enum Msg {
    Submit,
    Value(String),
}

pub struct Initialise {
    link: ComponentLink<Self>,
    ws_agent: Dispatcher<WebSocketAgent>,
    props: Props,
    value: String,
}

impl Component for Initialise {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws_agent: WebSocketAgent::dispatcher(),
            link,
            props,
            value: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                let session_id = self.props.session_id;
                let name = std::mem::take(&mut self.value);
                let request = Request::Post(Post::AddPlayer { session_id, name });
                self.ws_agent.send(request);
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

        let input = html! {
            <pbs::Input size=Size::Large r#type=InputType::Text placeholder={"eg. Alex"} value=self.value.clone() oninput=oninput/>
        };

        let button = html! {
            <pbs::Button size=Size::Large color=Color::Info onclick=onclick icon="fas fa-plus"/>
        };

        html! {
            <pbs::Field grouped=true>
                <pbs::Control expanded=true inner=input/>
                <pbs::Control inner=button/>
            </pbs::Field>
        }
    }
}
