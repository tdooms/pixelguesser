use std::rc::Rc;

use cobul::*;
use cropper::Cropper;
use shared::{callback, spawn};
use web_sys::HtmlImageElement;
use yew::*;

use api::{Image, Resolution, Round, Stage};
use components::Pixelate;
use components::{Center, DynImage, Fit, Height};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub round: Rc<Round>,
    pub edit: Callback<Rc<Round>>,
}

#[function_component(RoundPreview)]
pub fn round_preview(props: &Props) -> Html {
    let Props { round, edit } = props.clone();

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

    let change = callback!(pixels; move |new| pixels.set(new));
    let onslider = callback!(pixels; move |new| pixels.set(new));

    let onpause = callback!(stage; move |_| stage.set(Stage::Paused));
    let onrunning = callback!(stage; move |_| stage.set(Stage::Running));
    let reveal = callback!(stage; move |_| stage.set(Stage::Revealed));
    let onrevealing = callback!(stage; move |_| stage.set(Stage::Revealing));
    let cancel = callback!(cropper; move |_| cropper.set(false));
    let oncropper = callback!(cropper; move |_| cropper.set(true));

    let upload = callback!(round, edit; move |files: Vec<web_sys::File>| {
        let file = files[0].clone();
        spawn!(round, edit; async move {
            let image = Image::from_local(file).await.unwrap();
            edit.emit(Rc::new(Round{image, ..(*round).clone()}));
        })
    });
    let done = callback!(round, cropper; move |base64| {
        let image = Image::from_base64(base64, None);
        edit.emit(Rc::new(Round{image, ..(*round).clone()}));
        cropper.set(false);
    });

    let buttons = match *stage {
        Stage::Running => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            <simple::Button click={onrevealing} icon={fa::Solid::Forward} text="Reveal" />
            <simple::Button click={onpause} icon={fa::Solid::Pause} text="Pause" />
            </Buttons>
        },
        Stage::Paused => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            <simple::Button click={onrevealing} icon={fa::Solid::Forward} text="Reveal" />
            <simple::Button click={onrunning} icon={fa::Solid::Play} text="Play" />
            </Buttons>
        },
        Stage::Revealed => html! {
            <Buttons alignment={Alignment::Centered} class="mt-4">
            <simple::Button click={onrunning} icon={fa::Solid::Eye} text="Preview" />
            <simple::Button click={oncropper} icon={fa::Solid::Crop} text="Crop" />
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
                <Slider<u32> fullwidth=true value={*pixels} input={onslider} step=1 range={4..1000} label=true />
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
            <Cropper {src} {done} {cancel} height=450 width=600/>
        },
        (false, Stage::Revealed, false) => html! {
            <div>
                <DynImage {src} height={Height::Vh(85)} fit={Fit::Contain}/>
                {slider}
            </div>
        },
        (false, _, false) => html! {
            <div>
                <Pixelate {image} stage={*stage} {reveal} height=85 {change} pixels={*pixels}/>
                {slider}
            </div>
        },
        (true, _, false) => html! {
            <Center>
                <File accept={"image/*"} boxed=true alignment={Alignment::Centered} input={upload} />
            </Center>
        },
    };

    html! {
        <Column> {body} </Column>
    }
}
