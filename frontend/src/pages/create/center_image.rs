use yew::prelude::*;
use yewtil::NeqAssign;

use pbs::Alignment;

pub enum Msg {
    Upload(String),
    Reveal,
    Pause,
    Resume,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image_bytes: Option<Vec<u8>>,
}

pub struct CenterImage {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for CenterImage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
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
        match self.props.image_bytes.as_ref().map(base64::encode) {
            Some(base64) => html! {
                <>
                <cbs::DynImage src=format!("data:image/png;base64,{}", base64) height=85/>

                <pbs::Buttons alignment={Alignment::Centered} extra="mt-5">
                    <cbs::IconButton text="reveal" icon="fas fa-eye" onclick={self.link.callback(|_| Msg::Reveal)}/>
                    <cbs::IconButton text="resume" icon="fas fa-play" onclick={self.link.callback(|_| Msg::Resume)}/>
                    <cbs::IconButton text="pause" icon="fas fa-pause" onclick={self.link.callback(|_| Msg::Pause)}/>
                </pbs::Buttons>
                </>
            },
            None => html! {},
        }
    }
}
