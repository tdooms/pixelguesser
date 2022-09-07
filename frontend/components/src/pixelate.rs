use api::Stage;
use gloo::events::EventListener;
use gloo::timers::callback::Timeout;
use shared::pixelation::*;
use shared::{Level, Toast};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::*;
use ywt::{callback, clone};

#[derive(derive_more::Display, Debug)]
#[display(fmt = "Internal javascript draw error")]
enum Error {
    Js,
    Cast,
    Draw,
}

impl Toast for Error {
    fn level(&self) -> Level {
        Level::Error
    }
    fn leave(&self) -> bool {
        false
    }
}

fn canvas_context(element: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, Error> {
    Ok(element
        .get_context("2d")
        .map_err(|_| Error::Js)?
        .ok_or(Error::Cast)?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| Error::Cast)?)
}

fn draw_pixelated(
    pixels: u32,
    width: u32,
    height: u32,
    image: &HtmlImageElement,
    canvas: &HtmlCanvasElement,
    offscreen: &HtmlCanvasElement,
) -> Result<(), Error> {
    canvas.set_width(width);
    canvas.set_height(height);

    let aspect_ratio = image.width() as f64 / image.height() as f64;

    let (i_width, i_height) = (image.width() as f64, image.height() as f64);
    let (c_width, c_height) = (canvas.width() as f64, canvas.height() as f64);

    let scale = (c_width / i_width).min(c_height / i_height);

    let (x_pixels, y_pixels) = ((aspect_ratio * pixels as f64).round(), pixels as f64);

    let x_offset = (c_width - i_width * scale) / 2.0;
    let y_offset = (c_height - i_height * scale) / 2.0;

    let canvas_ctx = canvas_context(&canvas)?;
    let offscreen_ctx = canvas_context(&offscreen)?;

    offscreen_ctx
        .draw_image_with_html_image_element_and_dw_and_dh(&image, 0.0, 0.0, x_pixels, y_pixels)
        .map_err(|_| Error::Draw)?;

    canvas_ctx.set_image_smoothing_enabled(pixels > height);
    canvas_ctx.clear_rect(0.0, 0.0, c_width, c_height);

    Ok(canvas_ctx
        .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &offscreen,
            0.0,
            0.0,
            x_pixels,
            y_pixels,
            x_offset,
            y_offset,
            i_width * scale,
            i_height * scale,
        )
        .map_err(|_| Error::Draw)?)
}

fn update_duration(stage: Stage, pixels: u32, max: u32) -> Option<(f64, u32)> {
    let (speed, time) = match stage {
        Stage::Running => (PLAY_SPEED, PLAY_UPSCALE),
        Stage::Revealing => (REVEAL_SPEED, REVEAL_UPSCALE),
        _ => return None,
    };

    for new in pixels + 1..=max {
        let constant = time / ((START_PIXELS as f64 + 1.0) / START_PIXELS as f64).log(speed);
        let delay = constant * (new as f64 / pixels as f64).log(speed);

        if delay > MAX_REFRESH {
            return Some((delay, new));
        }
    }
    Some(((max as f64 / pixels as f64).log(speed), max))
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct Props {
    pub image: HtmlImageElement,
    pub stage: Stage,

    #[prop_or_default]
    pub reveal: Callback<()>,

    #[prop_or_default]
    pub change: Callback<u32>,

    #[prop_or_default]
    pub pixels: Option<u32>,

    #[prop_or(100)]
    pub height: u32,
}

#[function_component(Pixelate)]
pub fn pixelate(props: &Props) -> Html {
    let Props { image, stage, reveal, height, change, pixels } = props.clone();

    let window = web_sys::window().unwrap();
    let resizer = use_state(|| EventListener::new(&window, "resize", |_| ()));

    let counter = use_state_eq(|| 0);
    let onresize = callback!(counter; move |_| counter.set(*counter + 1));

    log::info!("{counter:?}");

    use_effect_with_deps(
        move |_| {
            let wrapper = move |_: &Event| onresize.emit(());
            resizer.set(EventListener::new(&window, "resize", wrapper));
            || ()
        },
        counter.clone(),
    );

    let size = use_state(|| START_PIXELS);
    let timer = use_state(|| Timeout::new(0, || ()));

    let canvas_ref = use_node_ref();
    let offscreen_ref = use_node_ref();
    let container_ref = use_node_ref();

    {
        let size = size.clone();
        use_effect_with_deps(
            move |pixels| {
                if let Some(pixels) = pixels {
                    size.set(*pixels);
                }
                || ()
            },
            pixels,
        );
    }

    {
        clone!(size, offscreen_ref, image, change);
        let src = image.src();
        // effect which initialises the canvas and state when the image is changed
        use_effect_with_deps(
            move |_| {
                let offscreen = offscreen_ref.cast::<HtmlCanvasElement>().unwrap();
                let (width, height) = (image.width(), image.height());

                offscreen.set_width(width);
                offscreen.set_height(height);

                change.emit(START_PIXELS);
                size.set(START_PIXELS);

                || ()
            },
            src,
        );
    }

    {
        clone!(timer, image, change);
        // effect which resets the timer when the size/stage changes
        use_effect_with_deps(
            move |(size, stage)| {
                match update_duration(*stage, **size, image.height()) {
                    Some((_, x)) if x == image.height() => {
                        reveal.emit(());
                    }
                    Some((time, new)) => {
                        clone!(size, change);
                        let cb = move || {
                            size.set(new);
                            change.emit(new)
                        };
                        timer.set(Timeout::new(time as u32, cb));
                    }
                    None => {
                        timer.set(Timeout::new(0, || ()));
                    }
                }

                || ()
            },
            (size.clone(), stage),
        )
    }

    {
        // effect which redraws the canvas when the size changes
        clone!(image, container_ref, canvas_ref, offscreen_ref);
        use_effect_with_deps(
            move |(size, _)| {
                let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
                let offscreen = offscreen_ref.cast::<HtmlCanvasElement>().unwrap();
                let container = container_ref.cast::<HtmlDivElement>().unwrap();

                let width = container.offset_width() as u32;
                let height = container.offset_height() as u32;

                log::warn!("redraw");
                draw_pixelated(**size, width, height, &image, &canvas, &offscreen).unwrap();
                || ()
            },
            (size.clone(), counter.clone()),
        );
    }

    html! {
        <div style={format!("height:{}vh", height)} ref={container_ref}>
            <canvas style="display:none" ref={offscreen_ref} />
            <canvas style="display:block" ref={canvas_ref} />
        </div>
    }
}
