use yew::prelude::*;
use yew::web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlImageElement};

use yew::utils::NeqAssign;

use crate::agents::NotificationAgent;
use crate::constants::IMAGE_ENDPOINT;
use crate::structs::Error;
use crate::utils::misc::draw_pixelated;
use crate::utils::yew::{Resizer, TypeRef};

use gloo::timers::callback::Timeout;
use shared::Status;
use yew_agent::{Dispatched, Dispatcher};

pub enum Msg {
    Loaded,
    Pixelate,
    Resize,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub onrevealed: Callback<()>,
    pub url: String,
    pub status: Status,
}

pub struct Pixelate {
    link: ComponentLink<Self>,
    props: Props,

    pixels: f64,

    resizer: Resizer,
    timer: Option<Timeout>,
    notifications: Dispatcher<NotificationAgent>,

    canvas: TypeRef<HtmlCanvasElement>,
    image: TypeRef<HtmlImageElement>,
    offscreen: TypeRef<HtmlCanvasElement>,
    container: TypeRef<HtmlDivElement>,
}

impl Pixelate {
    fn log(notifications: &mut Dispatcher<NotificationAgent>, result: Result<(), Error>) {
        if let Err(err) = result {
            // logger.send(err)
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
            resizer: Resizer::new(link.callback(|_| Msg::Resize)),
            link,
            pixels: 4.0,
            timer: None,
            notifications: NotificationAgent::dispatcher(),
            canvas: Default::default(),
            image: Default::default(),
            offscreen: Default::default(),
            container: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded => {
                let _ = self.initialize();
                let _ = self.draw();

                self.link.send_message(Msg::Pixelate);
            }
            Msg::Pixelate => {
                let speed = match self.props.status {
                    Status::Playing { paused: false } => 1.002,
                    Status::Revealing => 1.07,
                    Status::Revealed | Status::Playing { paused: true } => return false,
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
                    self.props.onrevealed.emit(());
                    self.timer = None;
                } else {
                    let cloned = self.link.clone();
                    self.timer = Some(Timeout::new(33, move || cloned.send_message(Msg::Pixelate)));
                }

                self.pixels = clamped_pixels;
                let _ = self.draw();
            }
            Msg::Resize => {
                let _ = self.draw();
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props.url != props.url {
            self.restart()
        }

        match props.status {
            Status::Playing { paused: true } => self.timer = None,
            Status::Playing { paused: false } => self.link.send_message(Msg::Pixelate),
            _ => {}
        };

        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <img src={format!("http://{}/{}", IMAGE_ENDPOINT, self.props.url)}
                     style="display:none"
                     onload={self.link.callback(|_| Msg::Loaded)}
                     ref={self.image.clone()}/>
                <canvas style="display:none" ref={self.offscreen.clone()}/>

                <div style="height:100vh" ref={self.container.clone()}>
                    <canvas style="display:block" ref={self.canvas.clone()}/>
                </div>
            </>
        }
    }
}
