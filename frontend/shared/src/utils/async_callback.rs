use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

pub fn async_callback<T: 'static>(fut: impl Future<Output = T> + 'static, callback: Callback<T>) {
    spawn_local(async move { callback.emit(fut.await) });
}
