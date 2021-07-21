use crate::agents::AlertAgent;
use crate::globals::IMAGE_ENDPOINT;
use crate::notifications::{Error, Notification};
use crate::utils::{draw_pixelated, TypeRef};
use api::Status;
use std::time::Duration;
use yew::agent::Dispatcher;
use yew::prelude::*;
use yew::web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
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
    pub on_revealed: Callback<()>,
    pub url: String,
    pub status: Status,
}

pub struct Pixelate {
    link: ComponentLink<Self>,
    logger: Dispatcher<AlertAgent>,

    _resizer: ResizeTask,
    props: Props,

    pixels: f64,
    timer: Option<TimeoutTask>,

    canvas: TypeRef<HtmlCanvasElement>,
    image: TypeRef<HtmlImageElement>,
    offscreen: TypeRef<HtmlCanvasElement>,
    container: TypeRef<HtmlDivElement>,
}

impl Pixelate {
    fn log(logger: &mut Dispatcher<AlertAgent>, result: Result<(), Error>) {
        if let Err(err) = result {
            logger.send(Notification::Error(err))
        }
    }

    fn initialize(&self) -> Result<(), Error> {
        let image = self.image.get()?;
        let offscreen = self.offscreen.get()?;

        offscreen.set_width(image.width());
        offscreen.set_height(image.height());

        Ok(())
    }

    fn draw(&mut self) -> Result<(), Error> {
        let image = self.image.get()?;
        let canvas = self.canvas.get()?;
        let offscreen = self.offscreen.get()?;
        let container = self.container.get()?;

        let width = container.offset_width() as u32;
        let height = container.offset_height() as u32;

        draw_pixelated(self.pixels as u32, width, height, image, canvas, offscreen)
    }

    pub fn restart(&mut self) {
        self.pixels = 4.0;
    }
}

impl Component for Pixelate {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            logger: AlertAgent::dispatcher(),
            _resizer: ResizeService::register(link.callback(|_| Msg::Resize)),
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
                let _ = self.initialize();
                let _ = self.draw();

                self.link.send_message(Msg::Pixelate);
                false
            }
            (Msg::Pixelate, Status::Playing | Status::Paused | Status::Revealing) => {
                // TODO: fix
                let speed = match self.props.status {
                    Status::Playing => 1.002,
                    _ => 1.07,
                };

                let max_pixels = match self.image.cast::<HtmlImageElement>() {
                    Some(image) => image.height() as f64,
                    None => 1080.0,
                };

                let new_pixels = self.pixels * speed;
                let clamped_pixels = new_pixels.min(max_pixels);

                // TODO: max pixels should be screen size instead of image size maybe?
                if clamped_pixels == max_pixels {
                    self.props.status = Status::Revealed;
                    self.props.on_revealed.emit(());
                    self.timer = None;
                } else {
                    let duration = Duration::from_millis(33);
                    let callback = self.link.callback(|_| Msg::Pixelate);
                    let task = TimeoutService::spawn(duration, callback);

                    self.timer = Some(task);
                }

                self.pixels = clamped_pixels;
                let _ = self.draw();

                false
            }
            (Msg::Resize, _) => {
                let _ = self.draw();
                false
            }
            _ => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.url != props.url {
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
