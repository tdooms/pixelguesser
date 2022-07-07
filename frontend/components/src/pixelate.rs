use api::{Image, Resolution, Stage};
use gloo::timers::callback::Timeout;
use shared::{
    Error, Internal, PIXELATE_PLAY_SPEED, PIXELATE_REFRESH_TIME, PIXELATE_REVEAL_SPEED,
    PIXELATE_START_PIXELS,
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::*;
use ywt::callback;

fn canvas_context(element: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, Error> {
    Ok(element
        .get_context("2d")
        .map_err(|_| Internal::JsError)?
        .ok_or(Internal::InvalidCast)?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| Internal::InvalidCast)?)
}

pub fn draw_pixelated(
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
        .map_err(|_| Internal::DrawError)?;

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
        .map_err(|_| Internal::DrawError)?)
}

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct Props {
    pub image: HtmlImageElement,
    pub stage: Stage,

    pub onreveal: Callback<()>,

    #[prop_or(100)]
    pub height: u32,
}

fn tick_size(current: f64, max: f64, stage: Stage) -> f64 {
    let new = match stage {
        Stage::Running => current * PIXELATE_PLAY_SPEED,
        Stage::Revealing => current * PIXELATE_REVEAL_SPEED,
        _ => current,
    };
    new.min(max)
}

#[function_component(Pixelate)]
pub fn pixelate(props: &Props) -> Html {
    let Props { image, stage, onreveal, height } = props.clone();

    let size = use_state(|| PIXELATE_START_PIXELS);
    let iteration = use_state(|| 0);
    let timer = use_state(|| Timeout::new(0, || ()));

    let canvas_ref = use_node_ref();
    let offscreen_ref = use_node_ref();
    let container_ref = use_node_ref();

    let (size_c, image_c) = (size.clone(), image.clone());
    use_effect_with_deps(
        move |iteration| {
            let new = tick_size(*size_c, image_c.height() as f64, stage);

            log::info!("pixel update tick: size = {new}");
            size_c.set(new);

            let cloned = iteration.clone();
            timer.set(Timeout::new(PIXELATE_REFRESH_TIME, move || cloned.set(*cloned + 1)));

            if new as u32 == image_c.height() {
                onreveal.emit(());
            }

            || ()
        },
        iteration.clone(),
    );

    let (image_c, container_c) = (image.clone(), container_ref.clone());
    let (canvas_c, offscreen_c) = (canvas_ref.clone(), offscreen_ref.clone());

    use_effect_with_deps(
        move |size| {
            log::info!("drawing");

            let canvas = canvas_c.cast::<HtmlCanvasElement>().unwrap();
            let offscreen = offscreen_c.cast::<HtmlCanvasElement>().unwrap();
            let container = container_c.cast::<HtmlDivElement>().unwrap();

            let width = container.offset_width() as u32;
            let height = container.offset_height() as u32;

            draw_pixelated(*size, width, height, &image_c, &canvas, &offscreen).unwrap();
            || ()
        },
        *size as u32,
    );

    let (size_c, offscreen_c, image_c) = (size.clone(), offscreen_ref.clone(), image.clone());
    use_effect_with_deps(
        move |_| {
            log::info!("changing images");

            let offscreen = offscreen_c.cast::<HtmlCanvasElement>().unwrap();
            let (width, height) = (image_c.width(), image_c.height());

            offscreen.set_width(width);
            offscreen.set_height(height);

            size_c.set(PIXELATE_START_PIXELS);
            || ()
        },
        image.clone(),
    );

    let style = format!("height:{}vh", height);
    html! {
        <div {style} ref={container_ref}>
            <canvas style="display:none" ref={offscreen_ref} />
            <canvas style="display:block" ref={canvas_ref} />
        </div>
    }
}
