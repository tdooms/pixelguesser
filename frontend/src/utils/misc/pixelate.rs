use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement};

use crate::structs::Error;

fn canvas_context(element: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, Error> {
    element
        .get_context("2d")
        .map_err(|_| Error::JsError)?
        .ok_or(Error::InvalidCast)?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| Error::InvalidCast)
}

pub fn draw_pixelated(
    pixels: u32,
    width: u32,
    height: u32,
    image: HtmlImageElement,
    canvas: HtmlCanvasElement,
    offscreen: HtmlCanvasElement,
) -> Result<(), Error> {
    canvas.set_width(width);
    canvas.set_height(height);

    let aspect_ratio = image.width() as f64 / image.height() as f64;

    let (i_width, i_height) = (image.width() as f64, image.height() as f64);
    let (c_width, c_height) = (canvas.width() as f64, canvas.height() as f64);

    let scale = (c_width / i_width).min(c_height / i_height);

    let x_pixels = (aspect_ratio * pixels as f64).round();
    let y_pixels = pixels as f64;

    let x_offset = (c_width - i_width * scale) / 2.0;
    let y_offset = (c_height - i_height * scale) / 2.0;

    let canvas_ctx = canvas_context(&canvas)?;
    let offscreen_ctx = canvas_context(&offscreen)?;

    offscreen_ctx
        .draw_image_with_html_image_element_and_dw_and_dh(&image, 0.0, 0.0, x_pixels, y_pixels)
        .map_err(|_| Error::DrawError)?;

    canvas_ctx.set_image_smoothing_enabled(pixels > height);
    canvas_ctx.clear_rect(0.0, 0.0, c_width, c_height);

    canvas_ctx
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
        .map_err(|_| Error::DrawError)
}
