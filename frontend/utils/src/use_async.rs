use std::default::Default;
use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::{use_state, UseStateHandle};

pub fn use_default_async_state<T: Default + 'static>(
    fut: impl Future<Output = T> + 'static,
) -> UseStateHandle<T> {
    let handle = use_state(T::default);
    let cloned = handle.clone();

    spawn_local(async move { cloned.set(fut.await) });

    handle
}
