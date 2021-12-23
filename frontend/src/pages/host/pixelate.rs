use gloo::timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};

use crate::agents::NotificationAgent;
use crate::constants::IMAGE_ENDPOINT;
use crate::structs::Error;
use crate::utils::misc::draw_pixelated;
use crate::utils::yew::{Resizer, TypeRef};

pub enum Msg {
    Loaded,
    Pixelate,
    Resize,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub revealing: bool,
    pub paused: bool,
}

pub struct Pixelate {
    url: String,
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
}

impl Component for Pixelate {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            resizer: Resizer::new(ctx.link().callback(|_| Msg::Resize)),
            url: ctx.props().url.clone(),
            pixels: 4.0,
            timer: None,
            notifications: NotificationAgent::dispatcher(),
            canvas: Default::default(),
            image: Default::default(),
            offscreen: Default::default(),
            container: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded => {
                let _ = self.initialize();
                let _ = self.draw();

                ctx.link().send_message(Msg::Pixelate);
            }
            Msg::Pixelate => {
                // Calculate the maximum pixels that are useful to de-pixelate
                let max_pixels = match self.image.get() {
                    Ok(image) => image.height() as f64,
                    Err(_) => 1080.0,
                };

                // Quick exit if already done
                if self.pixels >= max_pixels {
                    return false;
                }

                // Set the speed according to the state
                let speed = match (ctx.props().paused, ctx.props().revealing) {
                    (_, true) => 1.07,
                    (false, _) => 1.002,
                    _ => return false,
                };

                // Calculate the new amount of pixels and draw it
                self.pixels = (self.pixels * speed).min(max_pixels);
                let _ = self.draw();

                // Set the timer for the next iteration
                let cloned = ctx.link().clone();
                self.timer = Some(Timeout::new(33, move || cloned.send_message(Msg::Pixelate)));
            }
            Msg::Resize => {
                let _ = self.draw();
            }
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // If the url changes, reset the internal state
        if self.url != ctx.props().url {
            self.url = ctx.props().url.clone();
            self.pixels = 4.0
        }

        // Restart and remove the timer if needed
        match !ctx.props().paused || ctx.props().revealing {
            true => ctx.link().send_message(Msg::Pixelate),
            false => self.timer = None,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <img src={format!("{}/{}", IMAGE_ENDPOINT, ctx.props().url)}
                     style="display:none"
                     onload={ctx.link().callback(|_| Msg::Loaded)}
                     ref={self.image.clone()}/>

                <canvas style="display:none" ref={self.offscreen.clone()}/>

                <div style="height:100vh" ref={self.container.clone()}>
                    <canvas style="display:block" ref={self.canvas.clone()}/>
                </div>
            </>
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        log::error!("pixelate destructor");
        let canvas = self.canvas.get().unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        context.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64)
    }
}
