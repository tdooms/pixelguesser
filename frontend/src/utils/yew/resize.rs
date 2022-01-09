use gloo::events::EventListener;
use web_sys::{Event, Window};
use yew::Callback;

pub struct Dimensions {
    pub width: i32,
    pub height: i32,
}

impl Dimensions {
    pub fn from_window(window: &Window) -> Self {
        Self {
            width: window.inner_width().unwrap().as_f64().unwrap() as i32,
            height: window.inner_height().unwrap().as_f64().unwrap() as i32,
        }
    }
}

pub struct Resizer(EventListener);

impl Resizer {
    pub fn new(callback: Callback<Dimensions>) -> Self {
        let callback = move |_: &Event| {
            let dimensions = Dimensions::from_window(&web_sys::window().unwrap());
            callback.emit(dimensions)
        };

        let listener = EventListener::new(&web_sys::window().unwrap(), "resize", callback);
        Resizer(listener)
    }
}
