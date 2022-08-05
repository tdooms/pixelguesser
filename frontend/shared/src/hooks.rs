use crate::{use_toast, Toast};
use std::future::Future;
use yew::use_effect_with_deps;
use yew::*;

#[hook]
pub fn use_startup(func: impl FnOnce() + 'static) {
    use_effect_with_deps(
        move |_| {
            func();
            || ()
        },
        (),
    )
}

#[hook]
pub fn use_async_startup<F: Future<Output = Result<(), T>> + 'static, T: Toast + 'static>(fut: F) {
    let toast = use_toast();
    use_effect_with_deps(
        move |_| {
            ywt::spawn!(async move {
                match fut.await {
                    Ok(_) => (),
                    Err(err) => toast.add(err),
                }
            });
            || ()
        },
        (),
    )
}
