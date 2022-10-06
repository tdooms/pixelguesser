use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::*;

#[hook]
pub fn use_search<
    Q: Hash + Eq + Clone + Debug + 'static,
    T: PartialEq + 'static,
    R: Future<Output = (Vec<T>, Option<u64>)> + 'static,
    F: FnOnce(Q) -> R + 'static,
>(
    query: Q,
    gen: F,
) -> Option<Rc<(Vec<T>, Option<u64>)>> {
    let map = use_state_eq(|| HashMap::new());
    let timer = use_state(|| None);
    let prev = use_state(|| None);

    match map.get(&query) {
        Some(result) => return Some(Rc::clone(result)),
        None if (&*prev).as_ref() != Some(&query) => {
            prev.set(Some(query.clone()));

            let onquery = move || {
                ywt::spawn!(async move {
                    let result = gen(query.clone()).await;
                    let mut new = (*map).clone();

                    new.insert(query, Rc::new(result));
                    map.set(new);
                })
            };

            timer.set(Some(Timeout::new(1_000, onquery)));
        }
        _ => {}
    }

    None
}
