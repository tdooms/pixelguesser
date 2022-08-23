// use gloo::events::EventListener;
// use web_sys::{Event, Window};
// use yew::{hook, use_effect_with_deps, use_state, use_state_eq, Callback};
//
// #[derive(Debug)]
// pub struct Dimensions {
//     pub width: i32,
//     pub height: i32,
// }
//
// #[hook]
// pub fn use_resizer(callback: Callback<Dimensions>) -> EventListener {
//     use_effect_with_deps(move || {}, callback);
//     let window = web_sys::window().unwrap();
//
//     let cloned = window.clone();
//     let wrapper = move |_: &Event| {
//         let width = cloned.inner_width().unwrap().as_f64().unwrap() as i32;
//         let height = cloned.inner_height().unwrap().as_f64().unwrap() as i32;
//         callback.emit(Dimensions { width, height });
//     };
//
//     EventListener::new(&window, "resize", wrapper)
// }
