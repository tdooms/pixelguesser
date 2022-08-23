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

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { round, onedit } = props.clone();

    let stage = use_state(|| Stage::Revealed);
    let cropper = use_state(|| false);
    let pixels = use_state(|| 0);

    let cloned = stage.clone();
    use_effect_with_deps(
        move |_| {
            cloned.set(Stage::Revealed);
            || ()
        },
        props.round.image.clone(),
    );

    let onpixel = callback!(pixels; move |new| pixels.set(new));
    let onslider = callback!(pixels; move |new| pixels.set(new));

    let onpause = callback!(stage; move |_| stage.set(Stage::Paused));
    let onrunning = callback!(stage; move |_| stage.set(Stage::Running));
    let onreveal = callback!(stage; move |_| stage.set(Stage::Revealed));
    let onrevealing = callback!(stage; move |_| stage.set(Stage::Revealing));
    let oncancel = callback!(cropper; move |_| cropper.set(false));
    let oncropper = callback!(cropper; move |_| cropper.set(true));

    let onupload = callback!(round, onedit; move |files: Vec<web_sys::File>| {
        let file = files[0].clone();
        ywt::spawn!(round, onedit; async move {
            let image = Image::from_local(file).await.unwrap();
            onedit.emit(Rc::new(DraftRound{image, ..(*round).clone()}));
        })
    });
    let ondone = callback!(round, cropper; move |base64| {
        let image = Image::from_base64(base64, None);
        onedit.emit(Rc::new(DraftRound{image, ..(*round).clone()}));
        cropper.set(false);
    });

    let button = |onclick, icon, label| {
        html! {<Button {onclick}> <Icon {icon} /> <span> {label} </span> </Button>}
    };

    let buttons = match *stage {
        Stage::Running => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            {button(onrevealing, fa::Solid::Forward, "reveal")}
            {button(onpause, fa::Solid::Pause, "pause")}
            </Buttons>
        },
        Stage::Paused => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            {button(onrevealing, fa::Solid::Forward, "reveal")}
            {button(onrunning, fa::Solid::Play, "play")}
            </Buttons>
        },
        Stage::Revealed => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            {button(onrunning, fa::Solid::Eye, "preview")}
            {button(oncropper, fa::Solid::Crop, "crop")}
            </Buttons>
        },
        _ => html! {},
    };

    let slider = match *stage {
        Stage::Running | Stage::Paused | Stage::Revealing => html! {
            <Columns>
            <Column size={ColumnSize::IsNarrow}>
                {buttons}
            </Column>
            <Column>
                <Slider<u32> id="slideru" fullwidth=true value={*pixels} oninput={onslider} step=1 range={4..1000} label=true />
            </Column>
            </Columns>
        },
        _ => html! { buttons },
    };

    let src = round.image.src(Resolution::HD);
    let image = HtmlImageElement::new().unwrap();
    image.set_src(&src);

    let body = match (round.image.is_empty(), *stage, *cropper) {
        (_, _, true) => html! {
            <Cropper {src} {ondone} {oncancel} height=450 width=600/>
        },
        (false, Stage::Revealed, false) => html! {
            <div>
                <DynImage {src} height={Height::Vh(85)} fit={Fit::Contain}/>
                {slider}
            </div>
        },
        (false, _, false) => html! {
            <div>
                <Pixelate {image} stage={*stage} {onreveal} height=85 {onpixel} pixels={*pixels}/>
                {slider}
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
