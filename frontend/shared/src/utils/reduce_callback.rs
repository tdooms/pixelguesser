use yew::{Callback, Reducible, UseReducerHandle};

pub fn reduce_callback<S: Reducible + 'static, I: 'static>(
    handle: &UseReducerHandle<S>,
    func: impl Fn(I) -> S::Action + 'static,
) -> Callback<I> {
    let handle = handle.clone();
    Callback::from(move |x| handle.clone().dispatch(func(x)))
}
