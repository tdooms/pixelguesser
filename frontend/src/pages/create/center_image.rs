use yew::prelude::*;

use cobul::props::Alignment;
use cobul::*;

pub enum Msg {
    Reveal,
    Pause,
    Resume,
    Preview,
    Remove,
    Revealed,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub src: String,
    pub onremove: Callback<()>,
}

pub struct CenterImage {
    preview: bool,
}

impl Component for CenterImage {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { preview: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reveal => false,
            Msg::Pause => false,
            Msg::Resume => false,
            Msg::Preview => false,
            Msg::Remove => false,
            Msg::Revealed => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _pixelate_buttons = || {
            html! {
                <Buttons alignment={Alignment::Centered} extra="mt-5">
                    <Button onclick={ctx.link().callback(|_| Msg::Reveal)}>
                        <Icon icon={"fas fa-eye"} /> <span> {"reveal"} </span>
                    </Button>
                    <Button onclick={ctx.link().callback(|_| Msg::Resume)}>
                        <Icon icon={"fas fa-play"} /> <span> {"resume"} </span>
                    </Button>
                    <Button onclick={ctx.link().callback(|_| Msg::Pause)}>
                       <Icon icon={"fas fa-pause"} /> <span> {"pause"} </span>
                    </Button>
                </Buttons>
            }
        };

        html! {
            <>
                <DynImage src={ctx.props().src.clone()} height=85/>
                <Buttons alignment={Alignment::Centered} extra="mt-5">
                    <Button onclick={ctx.link().callback(|_| Msg::Preview)}>
                        <span> {"preview"} </span>
                    </Button>
                    <Button onclick={ctx.props().onremove.clone()}>
                        <Icon icon={"fas fa-trash"} /> <span> {"remove image"} </span>
                    </Button>
                </Buttons>
            </>
        }
    }
}
