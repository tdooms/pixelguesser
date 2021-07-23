use std::collections::HashMap;

use yew::prelude::*;
use yew::web_sys::File as SysFile;
use yewtil::NeqAssign;

use api::Round;

pub enum Msg {
    Upload(String),
    Reveal,
    Pause,
    Resume,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onupload: Callback<SysFile>,
    pub image: Option<String>,
}

pub struct SideImage {
    props: Props,
}

impl Component for SideImage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Upload(_) => false,
            Msg::Reveal => false,
            Msg::Pause => false,
            Msg::Resume => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.props.image {
            Some(image) => html! {
                <>
                <pbs::DynImage src={image} height=85/>

                <pbs::Buttons alignment={Alignment::Centered} extra="mt-5">
                    <pbs::Button text="reveal" icon="fas fa-eye" onclick={self.link.callback(|_| Msg::Reveal)}/>
                    <pbs::Button text="resume" icon="fas fa-play" onclick={self.link.callback(|_| Msg::Resume)}/>
                    <pbs::Button text="pause" icon="fas fa-pause" onclick={self.link.callback(|_| Msg::Pause)}/>
                </pbs::Buttons>
                </>
            },
            None => html! {
                <pbs::Center>
                    <pbs::File boxed=true alignment={Alignment::Centered} on_upload={self.link.callback(Msg::Upload)} />
                </pbs::Center>
            },
        };
    }
}
