use cobul::*;
use components::Pixelate;
use cropper::Cropper;
use std::rc::Rc;
use web_sys::HtmlImageElement;
use yew::*;

use api::{DraftRound, Image, Resolution, Stage};
use ywt::callback;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub draft: Rc<DraftRound>,
    pub onupload: Callback<Image>,
}

#[derive(Clone, Copy)]
enum State {
    Revealed,
    Revealing,
    Running,
    Paused,
}

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { draft, onupload } = props.clone();
    let state = use_state(|| State::Revealed);
    let cropper = use_state(|| false);

    let onpause = callback!(state; move |_| state.set(State::Paused));
    let onrunning = callback!(state; move |_| state.set(State::Running));
    let onreveal = callback!(state; move |_| state.set(State::Revealed));
    let onrevealing = callback!(state; move |_| state.set(State::Revealing));

    let cloned = state.clone();
    use_effect_with_deps(
        move |_| {
            cloned.set(State::Revealed);
            || ()
        },
        props.draft.image.clone(),
    );

    let ondone = callback!(cropper, onupload; move |base64| {
        onupload.emit(Image::from_base64(base64, None));
        cropper.set(false);
    });

    let oncancel = callback!(cropper; move |_| cropper.set(false));
    let oncropper = callback!(cropper; move |_| cropper.set(true));

    let onupload = callback!(onupload; move |files: Vec<web_sys::File>| {
        Image::from_file(files[0].clone(), onupload.clone());
    });

    let buttons = |idx: &[bool]| {
        html! {
            <Buttons alignment={Alignment::Centered} class="mt-5">
            <Button onclick={onrunning.clone()} hidden={idx[0]}>
                <Icon icon={Icons::EyeSolid} /> <span> {"preview"} </span>
            </Button>
            <Button onclick={oncropper} hidden={idx[1]}>
                <Icon icon={Icons::Crop} /> <span> {"crop"} </span>
            </Button>
            <Button onclick={onrevealing} hidden={idx[2]}>
                <Icon icon={Icons::Forward} /> <span> {"reveal"} </span>
            </Button>
            <Button onclick={onpause} hidden={idx[3]}>
                <Icon icon={Icons::Pause} /> <span> {"pause"} </span>
            </Button>
            <Button onclick={onrunning} hidden={idx[4]}>
                <Icon icon={Icons::Play} /> <span> {"continue"} </span>
            </Button>
            </Buttons>
        }
    };

    let (hidden, stage) = match *state {
        State::Revealing => ([true, true, true, true, true], Stage::Revealing),
        State::Running => ([true, true, false, false, true], Stage::Running),
        State::Paused => ([true, true, false, true, false], Stage::Paused),
        State::Revealed => ([false, false, true, true, true], Stage::Revealed),
    };

    let element = HtmlImageElement::new().unwrap();
    element.set_src(&draft.image.src(Resolution::HD));

    match (draft.image.is_none(), *state, *cropper) {
        (_, _, true) => html! {
            <Cropper src={draft.image.src(Resolution::Original)} {ondone} {oncancel} height=450 width=600/>
        },
        (false, State::Revealed, false) => html! {
            <div>
            <custom::DynImage src={draft.image.src(Resolution::HD)} height=85/>
            { buttons(&hidden) }
            </div>
        },
        (false, _, false) => html! {
            <div>
            <Pixelate image={element} {stage} {onreveal} height=85/>
            { buttons(&hidden) }
            </div>
        },
        (true, _, false) => html! {
            <custom::Center>
                <File accept={"image/*"} boxed=true alignment={Alignment::Centered} {onupload} />
            </custom::Center>
        },
    }
}
