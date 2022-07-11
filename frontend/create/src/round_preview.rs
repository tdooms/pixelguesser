use cobul::*;
use components::Pixelate;
use web_sys::HtmlImageElement;
use yew::*;

use api::{Image, Resolution, Stage};
use ywt::callback;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub image: Image,
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
    let Props { ref image, onupload } = props.clone();
    let state = use_state(|| State::Revealed);

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
        props.image.clone(),
    );

    let onupload = callback!(onupload; move |files: Vec<web_sys::File>| {
        Image::from_file(files[0].clone(), onupload.clone());
    });

    let buttons = |idx: &[bool]| {
        html! {
            <Buttons alignment={Alignment::Centered} class="mt-5">
            <Button onclick={onrunning.clone()} hidden={idx[0]}>
                <Icon icon={Icons::EyeSolid} /> <span> {"preview"} </span>
            </Button>
            <Button onclick={onrevealing} hidden={idx[1]}>
                <Icon icon={Icons::Forward} /> <span> {"reveal"} </span>
            </Button>
            <Button onclick={onpause} hidden={idx[2]}>
                <Icon icon={Icons::Pause} /> <span> {"pause"} </span>
            </Button>
            <Button onclick={onrunning} hidden={idx[3]}>
                <Icon icon={Icons::Play} /> <span> {"continue"} </span>
            </Button>
            </Buttons>
        }
    };

    let (hidden, stage) = match *state {
        State::Revealing => ([true, true, true, true], Stage::Revealing),
        State::Running => ([true, false, false, true], Stage::Running),
        State::Paused => ([true, false, true, false], Stage::Paused),
        State::Revealed => ([false, true, true, true], Stage::Revealed),
    };

    let element = HtmlImageElement::new().unwrap();
    element.set_src(&image.src(Resolution::HD));

    match (image.is_none(), *state) {
        (false, State::Revealed) => html! {
            <div>
            <custom::DynImage src={image.src(Resolution::HD)} height=85/>
            { buttons(&hidden) }
            </div>
        },
        (false, _) => html! {
            <div>
            <Pixelate image={element} {stage} {onreveal} height=85/>
            { buttons(&hidden) }
            </div>
        },
        (true, _) => html! {
            <custom::Center>
                <File accept={"image/*"} boxed=true alignment={Alignment::Centered} {onupload} />
            </custom::Center>
        },
    }
}
