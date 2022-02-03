use cobul::props::{Alignment, Color, Size};
use cobul::*;
use yew::prelude::*;

use crate::graphql::ImageData;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Option<ImageData>,

    pub onupload: Callback<Vec<web_sys::File>>,
    pub onremove: Callback<()>,
}

#[derive(Default, Clone)]
struct State {
    previewing: bool,
    revealing: bool,
    hovering: bool,
}

#[function_component(CenterSpace)]
pub fn center_space(props: &Props) -> Html {
    let Props { image, onupload, onremove } = props;
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

    let center = |image: &ImageData| match state.hovering {
        true => html! {
            <div>
                <DynImage src={image.src()} height=85 class="is-blurred"/>
                <div style="position:absolute;top:50%;left:50%;transform:translate(-50%, -50%);">
                    <Button color={Color::Danger} size={Size::Large} onclick={onremove}>{"remove"}</Button>
                </div>
            </div>
        },
        false => html! { <DynImage src={image.src()} height=85 class=""/> },
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
