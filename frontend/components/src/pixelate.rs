use api::{Image, Resolution, Stage};
use gloo::timers::callback::{Interval, Timeout};
use shared::{
    callback, Error, Internal, PIXELATE_PLAY_SPEED, PIXELATE_REFRESH_TIME, PIXELATE_REVEAL_SPEED,
    PIXELATE_START_PIXELS,
};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::*;

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
    pub image: Image,
    pub stage: Stage,

    pub onreveal: Callback<()>,

    #[prop_or(100)]
    pub height: u32,
}

#[function_component(Pixelate)]
pub fn pixelate(props: &Props) -> Html {
    let Props { image, stage, onreveal, height } = props.clone();

    let size = use_state(|| PIXELATE_START_PIXELS);
    let iteration = use_state(|| 0);

    let offscreen = use_node_ref();
    let canvas = use_node_ref();
    let container = use_node_ref();

    let image = use_state(|| {
        let element = web_sys::HtmlImageElement::new().unwrap();
        element.set_src(&image.src(Resolution::HD));
        element
    });

    let timer = use_state(|| Timeout::new(0, || ()));
    let image_height = image.height() as f64;
    let size_c = size.clone();
    use_effect_with_deps(
        move |iteration| {
            let new = match stage {
                Stage::Running => *size_c * PIXELATE_PLAY_SPEED,
                Stage::Revealing => *size_c * PIXELATE_REVEAL_SPEED,
                _ => *size_c,
            }
            .min(image_height);
            size_c.set(new);
            log::info!("{new}");

            if new == image_height {
                onreveal.emit(());
            } else {
                let cloned = iteration.clone();
                timer.set(Timeout::new(PIXELATE_REFRESH_TIME, move || cloned.set(*cloned + 1)));
            }

            || ()
        },
        iteration.clone(),
    );

    let (image_c, canvas_c, offscreen_c, container_c) =
        (image.clone(), canvas.clone(), offscreen.clone(), container.clone());

    use_effect_with_deps(
        move |size| {
            let element = container_c.cast::<HtmlDivElement>().unwrap();
            let width = element.offset_width() as u32;
            let height = element.offset_height() as u32;

            let canvas = canvas_c.clone().cast::<HtmlCanvasElement>().unwrap();
            let offscreen = offscreen_c.clone().cast::<HtmlCanvasElement>().unwrap();

            log::info!("drawing");
            draw_pixelated(*size, width, height, &*image_c, &canvas, &offscreen).unwrap();
            || ()
        },
        *size as u32,
    );

    let (size_c, offscreen_c, image_c) = (size.clone(), offscreen.clone(), image.clone());
    use_effect_with_deps(
        move |_| {
            let (width, height) = (image_c.width(), image_c.height());
            let offscreen = offscreen_c.cast::<HtmlCanvasElement>().unwrap();

            offscreen.set_width(width);
            offscreen.set_height(height);

            size_c.set(PIXELATE_START_PIXELS);
            || ()
        },
        image.clone(),
    );

    let style = format!("height:{}vh", height);
    html! {
        <div {style} ref={container.clone()}>
            <canvas style="display:none" ref={offscreen}/>
            <canvas style="display:block" ref={canvas}/>
        </div>
    }
}
