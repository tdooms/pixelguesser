use gloo::timers::callback::Timeout;
use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::rc::Rc;
use yew::*;

#[hook]
pub fn use_search<
    Q: Hash + Eq + Clone + 'static,
    T: 'static,
    R: Future<Output = Vec<T>>,
    F: Fn(Q) -> R + 'static,
>(
    query: Q,
    func: F,
) -> Option<Rc<Vec<T>>> {
    let map = use_state(|| HashMap::new());
    let timer = use_state(|| None);

    let cloned = map.clone();
    let query_c = query.clone();
    let onquery = move || {
        ywt::spawn!(async move {
            let result = func(query_c.clone()).await;
            let mut new = (*cloned).clone();

            new.insert(query_c, Rc::new(result));
            cloned.set(new);
        })
    };

    match map.get(&query) {
        Some(result) => return Some(Rc::clone(result)),
        None => timer.set(Some(Timeout::new(1_000, onquery))),
    }

    None
}
