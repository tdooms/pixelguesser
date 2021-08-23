use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::Alignment;
use yew::utils::NeqAssign;

pub enum Msg {
    Reveal,
    Pause,
    Resume,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image_url: String,
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
            Msg::Reveal => false,
            Msg::Pause => false,
            Msg::Resume => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            <cbs::DynImage src={self.props.image_url.clone()} height=85/>

            <Buttons alignment={Alignment::Centered} extra="mt-5">
                <Button onclick={self.link.callback(|_| Msg::Reveal)}>
                    <span class="icon"> <Icon icon={"fas fa-eye"} /> </span> <span> {"reveal"} </span>
                </Button>
                <Button onclick={self.link.callback(|_| Msg::Resume)}>
                    <span class="icon"> <Icon icon={"fas fa-play"} /> </span> <span> {"resume"} </span>
                </Button>
                <Button onclick={self.link.callback(|_| Msg::Pause)}>
                    <span class="icon"> <Icon icon={"fas fa-pause"} /> </span> <span> {"pause"} </span>
                </Button>
            </Buttons>
            </>
        }
    }
}
