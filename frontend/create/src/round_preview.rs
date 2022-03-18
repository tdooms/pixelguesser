use cobul::props::{Alignment, Color, Size};
use cobul::*;
use yew::prelude::*;

use api::{Image, Resolution};
use shared::{async_callback, callback};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Option<api::Image>,

    pub onupload: Callback<Image>,
    pub onremove: Callback<()>,
}

#[derive(Default, Clone, Copy)]
struct State {
    previewing: bool,
    revealing: bool,
    hovering: bool,
}

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { image, onupload, onremove } = props;
    let state = use_state(|| State::default());

    let onpreview = callback!(state; move |_| state.set(State{previewing: true, ..*state}));
    let onhover = |hovering| callback!(state; move |_| state.set(State{hovering, ..*state}));

    let onupload = {
        Callback::from(|files: Vec<web_sys::File>| {
            async_callback(Image::from_local(files[0]), onupload.clone());
        })
    };

    let buttons = html! {
        <Button onclick={onpreview}>
            <Icon icon={Icons::EyeSolid} /> <span> {"preview"} </span>
        </Button>
    };

    let center = |image: &api::Image| match state.hovering {
        true => html! {
            <div>
                <DynImage src={image.src(Resolution::FullHd)} height=85 class="is-blurred"/>
                <div style="position:absolute;top:50%;left:50%;transform:translate(-50%, -50%);">
                    <Button color={Color::Danger} size={Size::Large} onclick={onremove}>{"remove"}</Button>
                </div>
            </div>
        },
        false => html! { <DynImage src={image.src(Resolution::FullHd)} height=85 class=""/> },
    };

    match &image {
        Some(image) => html! {
            <>
            <div onmouseenter={onhover(true)} onmouseleave={onhover(false)}>
                { center(image) }
            </div>
            <Buttons alignment={Alignment::Centered} class="mt-5">
                { buttons }
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
