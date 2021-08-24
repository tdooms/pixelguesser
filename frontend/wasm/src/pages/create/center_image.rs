use yew::prelude::*;
use yew::utils::NeqAssign;

use pbs::prelude::*;
use pbs::properties::Alignment;

pub enum Msg {
    Reveal,
    Pause,
    Resume,
    Preview,
    Remove,
    Revealed
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub src: String,
    pub onremove: Callback<()>,
}

pub struct CenterImage {
    props: Props,
    link: ComponentLink<Self>,

    preview: bool,
}

impl Component for CenterImage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, preview: false }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Reveal => false,
            Msg::Pause => false,
            Msg::Resume => false,
            Msg::Preview => false,
            Msg::Remove => false,
            Msg::Revealed => false
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {

        let pixelate_buttons = || html!{
            <Buttons alignment={Alignment::Centered} extra="mt-5">
                <Button onclick={self.link.callback(|_| Msg::Reveal)}>
                    <Icon icon={"fas fa-eye"} /> <span> {"reveal"} </span>
                </Button>
                <Button onclick={self.link.callback(|_| Msg::Resume)}>
                    <Icon icon={"fas fa-play"} /> <span> {"resume"} </span>
                </Button>
                <Button onclick={self.link.callback(|_| Msg::Pause)}>
                   <Icon icon={"fas fa-pause"} /> <span> {"pause"} </span>
                </Button>
            </Buttons>
        };

        html! {
            <>
                <cbs::DynImage src={self.props.src.clone()} height=85/>
                <Buttons alignment={Alignment::Centered} extra="mt-5">
                    <Button onclick={self.link.callback(|_| Msg::Preview)}>
                        <span> {"preview"} </span>
                    </Button>
                    <Button onclick={self.props.onremove.clone()}>
                        <Icon icon={"fas fa-trash"} /> <span> {"remove image"} </span>
                    </Button>
                </Buttons>
            </>
        }
    }
}
