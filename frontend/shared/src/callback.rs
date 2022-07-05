use std::future::Future;
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Reducible, UseReducerHandle};

#[macro_export]
macro_rules! callback {
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            Callback::from($y)
        }
    };
}

pub fn async_callback<T: 'static>(fut: impl Future<Output = T> + 'static, callback: Callback<T>) {
    spawn_local(async move { callback.emit(fut.await) });
}

pub fn reduce_callback<S: Reducible + 'static, I: 'static>(
    handle: &UseReducerHandle<S>,
    func: impl Fn(I) -> S::Action + 'static,
) -> Callback<I> {
    let handle = handle.clone();
    Callback::from(move |x| handle.clone().dispatch(func(x)))
}
