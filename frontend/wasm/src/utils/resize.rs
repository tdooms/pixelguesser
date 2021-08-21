use gloo::events::EventListener;
use yew::Callback;
use web_sys::{Window, Event};

pub struct Dimensions {
    width: i32,
    height: i32,
}

impl Dimensions {
    pub fn from_window(window: &Window) -> Self {
        Self {
            width: window.inner_width(),
            height: window.inner_height(),
        }
    }
}

pub struct Resizer(EventListener);

impl Resizer {
    pub fn new(callback: Callback<Dimensions>) -> Self {
        let callback = move |_: Event| {
            let dimensions = Dimensions::from_window(web_sys::window().unwrap());
            callback.emit(dimensions)
        };

        let listener = EventListener::new(web_sys::window().unwrap(), "resize", callback);
        Resizer(listener)
    }
}

