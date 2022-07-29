use cobul::*;
use components::Pixelate;
use components::{Center, DynImage, Fit, Height};
use cropper::Cropper;
use std::rc::Rc;
use web_sys::HtmlImageElement;
use yew::*;

use api::{DraftRound, Image, Resolution, Stage};
use ywt::callback;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub round: Rc<DraftRound>,
    pub onedit: Callback<Rc<DraftRound>>,
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
    let Props { round, onedit } = props.clone();

    let state = use_state(|| State::Revealed);
    let cropper = use_state(|| false);

    let cloned = state.clone();
    use_effect_with_deps(
        move |_| {
            cloned.set(State::Revealed);
            || ()
        },
        props.round.image.clone(),
    );

    let onpause = callback!(state; move |_| state.set(State::Paused));
    let onrunning = callback!(state; move |_| state.set(State::Running));
    let onreveal = callback!(state; move |_| state.set(State::Revealed));
    let onrevealing = callback!(state; move |_| state.set(State::Revealing));
    let oncancel = callback!(cropper; move |_| cropper.set(false));
    let oncropper = callback!(cropper; move |_| cropper.set(true));

    let onupload = callback!(round, onedit; move |files: Vec<web_sys::File>| {
        let file = files[0].clone();
        ywt::spawn!(round, onedit; async move {
            let image = Image::from_web(file).await.unwrap();
            onedit.emit(Rc::new(DraftRound{image, ..(*round).clone()}));
        })
    });
    let ondone = callback!(round, cropper; move |base64| {
        let image = Image::from_base64(base64, None);
        onedit.emit(Rc::new(DraftRound{image, ..(*round).clone()}));
        cropper.set(false);
    });

    let buttons = |idx: &[bool]| {
        html! {
            <Buttons alignment={Alignment::Centered} class="mt-5">
            <Button onclick={onrunning.clone()} hidden={idx[0]}>
                <Icon icon={fa::Solid::Eye} /> <span> {"preview"} </span>
            </Button>
            <Button onclick={oncropper} hidden={idx[1]}>
                <Icon icon={fa::Solid::Crop} /> <span> {"crop"} </span>
            </Button>
            <Button onclick={onrevealing} hidden={idx[2]}>
                <Icon icon={fa::Solid::Forward} /> <span> {"reveal"} </span>
            </Button>
            <Button onclick={onpause} hidden={idx[3]}>
                <Icon icon={fa::Solid::Pause} /> <span> {"pause"} </span>
            </Button>
            <Button onclick={onrunning} hidden={idx[4]}>
                <Icon icon={fa::Solid::Play} /> <span> {"continue"} </span>
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

    let src = round.image.src(Resolution::HD);
    let image = HtmlImageElement::new().unwrap();
    image.set_src(&src);

    let body = match (round.image.is_empty(), *state, *cropper) {
        (_, _, true) => html! {
            <Cropper {src} {ondone} {oncancel} height=450 width=600/>
        },
        (false, State::Revealed, false) => html! {
            <div>
            <DynImage {src} height={Height::Vh(85)} fit={Fit::Contain}/>
            { buttons(&hidden) }
            </div>
        },
        (false, _, false) => html! {
            <div>
            <Pixelate {image} {stage} {onreveal} height=85/>
            { buttons(&hidden) }
            </div>
        },
        (true, _, false) => html! {
            <Center>
                <File accept={"image/*"} boxed=true alignment={Alignment::Centered} {onupload} />
            </Center>
        },
    };

    html! {
        <Column> {body} </Column>
    }
}
