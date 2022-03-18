use cobul::props::{Alignment, Color, Size};
use cobul::*;
use shared::async_callback;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use api::Resolution;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Option<api::Image>,

    pub onupload: Callback<api::Image>,
    pub onremove: Callback<()>,
}

#[derive(Default, Clone)]
struct State {
    previewing: bool,
    revealing: bool,
    hovering: bool,
}

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { image, onupload, onremove } = props.clone();
    let state = use_state(|| State::default());

    let onpreview = {
        let cloned = state.clone();
        Callback::from(move |_| {
            cloned.set(State { previewing: true, revealing: true, hovering: false })
        })
    };

    let onhover = |hovering| {
        let cloned = state.clone();
        Callback::from(move |_| cloned.set(State { hovering, ..*cloned }))
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

    let onenter = Callback::from(move |files: Vec<web_sys::File>| {
        async_callback(api::Image::from_local(files[0].clone()), onupload.clone())
    });

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
                <File boxed=true alignment={Alignment::Centered} onupload={onenter} />
            </Center>
        },
    }
}
