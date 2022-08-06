use crate::{use_toast, Generic};
use gloo::timers::callback::Timeout;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::rc::Rc;
use yew::*;

#[hook]
pub fn use_search<
    Q: Hash + Eq + Clone + Default + Debug + 'static,
    T: PartialEq + 'static,
    R: Future<Output = (Vec<T>, Option<u64>)> + 'static,
    F: FnOnce(Q) -> R + 'static,
>(
    query: Q,
    gen: F,
) -> Option<Rc<(Vec<T>, Option<u64>)>> {
    let map = use_state_eq(|| HashMap::new());
    let timer = use_state(|| None);
    let prev = use_state(|| Q::default());
    let toast = use_toast();

    match map.get(&query) {
        Some(result) => return Some(Rc::clone(result)),
        None if &*prev != &query => {
            prev.set(query.clone());

            let onquery = move || {
                ywt::spawn!(async move {
                    toast.add(Generic::info(format!("Query for {query:?}"), false));

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
