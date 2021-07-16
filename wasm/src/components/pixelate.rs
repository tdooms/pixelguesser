use crate::agents::{NotifyAgent, WebSocketAgent};
use crate::globals::IMAGE_ENDPOINT;
use crate::notifications::{Error, Notification};
use api::{Post, Request, Stage, Status};
use std::time::Duration;
use wasm_bindgen::JsCast;
use yew::agent::Dispatcher;
use yew::prelude::*;
use yew::web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::Properties;
use yew_services::resize::ResizeTask;
use yew_services::timeout::TimeoutTask;
use yew_services::{ResizeService, TimeoutService};

pub enum Msg {
    Loaded,
    Pixelate,
    Resize,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub url: String,
    pub status: Status,

    pub round: usize,
    pub session_id: u64,
}

pub struct Pixelate {
    link: ComponentLink<Self>,
    ws_agent: Dispatcher<WebSocketAgent>,
    log_agent: Dispatcher<NotifyAgent>,

    _resizer: ResizeTask,
    props: Props,

    pixels: f64,
    timer: Option<TimeoutTask>,

    canvas: NodeRef,
    image: NodeRef,
    offscreen: NodeRef,
    container: NodeRef,
}

impl Pixelate {
    fn canvas_context(element: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, Error> {
        element
            .get_context("2d")
            .map_err(|_| Error::PixelationCanvasError)?
            .ok_or(Error::PixelationCanvasError)?
            .dyn_into::<CanvasRenderingContext2d>()
            .map_err(|_| Error::PixelationCanvasError)
    }

    fn get_canvasses(
        &self,
    ) -> Result<(HtmlImageElement, HtmlCanvasElement, HtmlCanvasElement), Error> {
        let image = self.image.cast::<HtmlImageElement>();
        let canvas = self.canvas.cast::<HtmlCanvasElement>();
        let offscreen = self.offscreen.cast::<HtmlCanvasElement>();

        match (image, canvas, offscreen) {
            (Some(x), Some(y), Some(z)) => Ok((x, y, z)),
            _ => Err(Error::PixelationCanvasError),
        }
    }

    fn initialize_canvas(&self) -> Result<(), Error> {
        let (image, canvas, offscreen) = self.get_canvasses()?;

        offscreen.set_width(image.width());
        offscreen.set_height(image.height());

        let canvas_ctx = Self::canvas_context(&canvas)?;
        canvas_ctx.set_image_smoothing_enabled(false);

        Ok(())
    }

    fn initialize(&mut self) {
        if let Err(err) = self.initialize_canvas() {
            self.log_agent.send(Notification::Error(err))
        }
    }

    fn draw_pixelated(&self) -> Result<(), Error> {
        let pixels = self.pixels as u32;
        let (image, canvas, offscreen) = self.get_canvasses()?;

        let container = self
            .container
            .cast::<HtmlDivElement>()
            .ok_or(Error::PixelationCanvasError)?;

        canvas.set_width(container.offset_width() as u32);
        canvas.set_height(container.offset_height() as u32);

        let aspect_ratio = image.width() as f64 / image.height() as f64;

        let i_width = image.width() as f64;
        let i_height = image.height() as f64;

        let c_width = canvas.width() as f64;
        let c_height = canvas.height() as f64;

        let scale = (c_width / i_width).min(c_height / i_height);

        let x_pixels = (aspect_ratio * pixels as f64).round();
        let y_pixels = pixels as f64;

        let x_offset = (c_width - i_width * scale) / 2.0;
        let y_offset = (c_height - i_height * scale) / 2.0;

        let canvas_ctx = Self::canvas_context(&canvas)?;
        let offscreen_ctx = Self::canvas_context(&offscreen)?;

        offscreen_ctx
            .draw_image_with_html_image_element_and_dw_and_dh(&image, 0.0, 0.0, x_pixels, y_pixels)
            .map_err(|_| Error::PixelationDrawError)?;

        canvas_ctx.set_image_smoothing_enabled(self.props.status == Status::Revealed);
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
            .map_err(|_| Error::PixelationDrawError)
    }

    fn draw(&mut self) {
        if let Err(err) = self.draw_pixelated() {
            self.log_agent.send(Notification::Error(err))
        }
    }

    pub fn restart(&mut self) {
        self.pixels = 4.0;
    }
}

impl Component for Pixelate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let resizer = ResizeService::register(link.callback(|_| Msg::Resize));

        Self {
            props,
            ws_agent: WebSocketAgent::dispatcher(),
            log_agent: NotifyAgent::dispatcher(),
            _resizer: resizer,
            link,
            pixels: 4.0,
            timer: None,
            canvas: Default::default(),
            image: Default::default(),
            offscreen: Default::default(),
            container: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match (msg, self.props.status) {
            (Msg::Loaded, _) => {
                self.initialize();
                self.draw();

                self.link.send_message(Msg::Pixelate);
                false
            }
            (Msg::Pixelate, Status::Playing | Status::Paused | Status::Revealing) => {
                let speed = match self.props.status {
                    Status::Playing => 1.002,
                    Status::Revealing => 1.07,
                    _ => return false,
                };

                let max_pixels = match self.image.cast::<HtmlImageElement>() {
                    Some(image) => image.height() as f64,
                    None => 1080.0,
                };

                let new_pixels = self.pixels * speed;
                let clamped_pixels = new_pixels.min(max_pixels);

                if clamped_pixels == max_pixels {
                    self.props.status = Status::Revealed;
                    let Props {
                        round, session_id, ..
                    } = self.props;

                    let stage = Stage::Round {
                        round,
                        status: Status::Revealed,
                    };

                    let request = Request::Post(Post::ChangeStage { session_id, stage });
                    self.ws_agent.send(request);
                    self.timer = None;
                } else {
                    let duration = Duration::from_millis(33);
                    let callback = self.link.callback(|_| Msg::Pixelate);
                    let task = TimeoutService::spawn(duration, callback);

                    self.timer = Some(task);
                }

                self.pixels = clamped_pixels;
                self.draw();

                false
            }
            (Msg::Resize, _) => {
                self.draw();
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.round != props.round {
            self.restart()
        }
        if let (Some(_), Status::Paused) = (self.timer.as_ref(), props.status) {
            self.timer = None;
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <img src={format!("http://{}/{}", IMAGE_ENDPOINT, self.props.url)}
                     style="display:none"
                     onload=self.link.callback(|_| Msg::Loaded)
                     ref=self.image.clone()/>
                <canvas style="display:none" ref=self.offscreen.clone()/>

                <div style="height:100vh" ref=self.container.clone()>
                    <canvas style="display:block" ref=self.canvas.clone()/>
                </div>
            </>
        }
    }
}
