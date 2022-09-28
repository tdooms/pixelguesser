use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use validator::Validate;
pub use yew::*;

#[derive(Clone)]
pub struct UseFormHandle<T: Clone> {
    errors: HashMap<String, String>,
    state: Rc<T>,
    change: Callback<Rc<T>>,
}

impl<T: Clone> Deref for UseFormHandle<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: Clone + 'static> UseFormHandle<T> {
    pub fn error(&self, key: &str) -> Option<String> {
        self.errors.get(&key.to_owned()).cloned()
    }

    pub fn errors(&self) -> &HashMap<String, String> {
        &self.errors
    }

    pub fn change<F>(&self, mapper: impl Fn(&mut T) -> &mut F + 'static) -> Callback<F> {
        let state = self.state.clone();

        self.change.reform(move |value| {
            let mut new = (*state).clone();
            *mapper(&mut new) = value;
            Rc::new(new)
        })
    }
}

#[hook]
pub fn use_form<T: Clone + Validate>(state: Rc<T>, change: Callback<Rc<T>>) -> UseFormHandle<T> {
    let errors = match state.as_ref().validate() {
        Ok(_) => return UseFormHandle { errors: HashMap::new(), state, change },
        Err(errors) => errors,
    };

    // SAFETY: Here the assumption is made that none of the vectors in the map are empty.
    let errors = errors
        .field_errors()
        .into_iter()
        .map(|(field, vec)| (field.to_owned(), vec.first().unwrap().to_string()))
        .collect();

    UseFormHandle { errors, state, change }
}
