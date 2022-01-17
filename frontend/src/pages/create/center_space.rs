use crate::graphql::ImageData;
use cobul::props::{Alignment, Color};
use cobul::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Option<ImageData>,

    pub onupload: Callback<Vec<web_sys::File>>,
    pub onremove: Callback<()>,
}

#[derive(Default)]
struct State {
    previewing: bool,
    revealing: bool,
}

#[function_component(CenterSpace)]
pub fn center_space(props: &Props) -> Html {
    let Props { image, onupload, onremove } = props;
    let state = use_state(|| State::default());

    let onpreview = {
        let cloned = state.clone();
        Callback::from(move |_| cloned.set(State { previewing: true, revealing: true }))
    };

    // let _pixelate_buttons = || {
    //     html! {
    //         <Buttons alignment={Alignment::Centered} extra="mt-5">
    //             <Button onclick={ctx.link().callback(|_| Msg::Reveal)}>
    //                 <Icon icon={"fas fa-eye"} /> <span> {"reveal"} </span>
    //             </Button>
    //             <Button onclick={ctx.link().callback(|_| Msg::Resume)}>
    //                 <Icon icon={"fas fa-play"} /> <span> {"resume"} </span>
    //             </Button>
    //             <Button onclick={ctx.link().callback(|_| Msg::Pause)}>
    //                <Icon icon={"fas fa-pause"} /> <span> {"pause"} </span>
    //             </Button>
    //         </Buttons>
    //     }
    // };

    match &image {
        Some(image) => html! {
            <>
            <DynImage src={image.src()} height=85/>
            <Buttons alignment={Alignment::Centered} extra="mt-5">
                <Button onclick={onremove} light=true color={Color::Danger}>
                    <Icon icon={Icons::Trash} /> <span> {"remove image"} </span>
                </Button>
                <Button onclick={onpreview}>
                    <Icon icon={Icons::EyeSolid} /> <span> {"preview"} </span>
                </Button>
            </Buttons>
            </>
        },
        None => html! {
            <Center>
                <File boxed=true alignment={Alignment::Centered} onupload={onupload} />
            </Center>
        },
    }
}
