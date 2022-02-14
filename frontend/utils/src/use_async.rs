use std::future::Future;
use wasm_bindgen_futures::spawn_local;

pub fn use_async<F, T>(fut: impl Future<Output = T> + 'static, func: impl FnOnce(T) + 'static) {
    spawn_local(async move {
        func(fut.await);
    })
}
