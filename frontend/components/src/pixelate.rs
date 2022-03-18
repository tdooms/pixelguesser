use gloo::timers::callback::Timeout;
use web_sys::{HtmlCanvasElement, HtmlDivElement, HtmlImageElement};
use yew::prelude::*;

use api::IMAGE_ENDPOINT;
use shared::{
    draw_pixelated, set_timer, EmitError, Error, Resizer, TypeRef, PIXELATE_PLAY_SPEED,
    PIXELATE_REFRESH_TIME, PIXELATE_REVEAL_SPEED, PIXELATE_START_PIXELS,
};

#[derive(Debug)]
pub enum Msg {
    Loaded,
    Pixelate,
    Resize,
    CouldNotLoad,
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct Props {
    pub url: String,
    pub revealing: bool,
    pub paused: bool,
    pub onrevealed: Callback<()>,
}

pub struct Pixelate {
    old: Props,
    pixels: f64,

    _resizer: Resizer,

    timer: Option<Timeout>,

    canvas: TypeRef<HtmlCanvasElement>,
    image: TypeRef<HtmlImageElement>,
    offscreen: TypeRef<HtmlCanvasElement>,
    container: TypeRef<HtmlDivElement>,
}

impl Pixelate {
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

    fn pixelate(&mut self, ctx: &Context<Self>) -> Result<(), Error> {
        let Props { revealing, paused, onrevealed, .. } = ctx.props();
        self.timer = None;

        // Calculate the maximum pixels that are useful to de-pixelate
        let max_pixels = self.image.get().map(|x| x.height() as f64).unwrap_or(1080.0);

        // Quick exit if already done
        if self.pixels >= max_pixels {
            onrevealed.emit(());
            return Ok(());
        }

        // Set the speed according to the state
        let speed = match (paused, revealing) {
            (_, true) => PIXELATE_REVEAL_SPEED,
            (false, _) => PIXELATE_PLAY_SPEED,
            _ => return Ok(()),
        };

        // Calculate the new amount of pixels and draw it
        self.pixels = (self.pixels * speed).min(max_pixels);
        self.draw()?;

        // Set the timer for the next iteration
        self.timer = Some(set_timer(ctx.link(), PIXELATE_REFRESH_TIME, Msg::Pixelate));
        Ok(())
    }
}

impl Component for Pixelate {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("pixelate create");
        Self {
            _resizer: Resizer::new(ctx.link().callback(|_| Msg::Resize)),
            old: ctx.props().clone(),
            pixels: PIXELATE_START_PIXELS,
            timer: None,
            canvas: Default::default(),
            image: Default::default(),
            offscreen: Default::default(),
            container: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (errors, _) = ctx.link().context(Callback::noop()).unwrap();

        match msg {
            Msg::Loaded => {
                // Initialize and draw each time at the start to avoid graphical glitches
                self.initialize().emit(&errors);
                self.draw().emit(&errors);

                // Start pixelating
                ctx.link().send_message(Msg::Pixelate)
            }
            Msg::Pixelate => {
                // Pixelate loop
                self.pixelate(ctx).emit(&errors);
            }
            Msg::Resize => {
                // Redraw on resize to reduce stutter
                self.draw().emit(&errors);
            }
            Msg::CouldNotLoad => errors.emit(Error::ImageCouldNotLoad),
        }
        false
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        // If the url changes, reset the internal state
        if self.old.url != ctx.props().url {
            self.pixels = PIXELATE_START_PIXELS
        }

        // Check if we need to start re-pixelating
        let pause_change = self.old.paused != ctx.props().paused;
        let reveal_change = self.old.revealing != ctx.props().revealing;
        if let (None, true) = (&self.timer, pause_change || reveal_change) {
            ctx.link().send_message(Msg::Pixelate)
        }

        self.old = ctx.props().clone();
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <img style="display:none"
                    src={format!("{IMAGE_ENDPOINT}/{}", ctx.props().url)}
                    onload={ctx.link().callback(|_| Msg::Loaded)}
                    onerror={ctx.link().callback(|_| Msg::CouldNotLoad)}
                    ref={self.image.clone()} />

                <canvas style="display:none" ref={self.offscreen.clone()}/>

                <div style="height:100vh" ref={self.container.clone()}>
                    <canvas style="display:block" ref={self.canvas.clone()}/>
                </div>
            </>
        }
    }
}
