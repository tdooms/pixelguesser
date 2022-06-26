use cobul::props::Alignment;
use cobul::*;
use yew::prelude::*;

use api::{Image, Resolution};
use shared::{async_callback, callback};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Option<api::Image>,
    pub onupload: Callback<Image>,
}

#[derive(Default, Clone, Copy)]
struct State {
    previewing: bool,
    revealing: bool,
}

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { ref image, onupload } = props.clone();
    let state = use_state(|| State::default());

    let onpreview = callback!(state; move |_| state.set(State{previewing: true, ..*state}));

    let onupload = {
        Callback::from(move |files: Vec<web_sys::File>| {
            async_callback(Image::from_local(files[0].clone()), onupload.clone());
        })
    };

    let buttons = html! {
        <Button onclick={onpreview}>
            <Icon icon={Icons::EyeSolid} /> <span> {"preview"} </span>
        </Button>
    };

    match &image {
        Some(image) => html! {
            <>
            <div>
                <DynImage src={image.src(Resolution::FullHd)} height=85 class=""/>
            </div>
            <Buttons alignment={Alignment::Centered} class="mt-5">
                { buttons }
            </Buttons>
            </>
        },
        None => html! {
            <Center>
                <File accept={"image/*"} boxed=true alignment={Alignment::Centered} onupload={onupload} />
            </Center>
        },
    }
}
