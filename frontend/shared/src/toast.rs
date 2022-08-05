use gloo::timers::callback::Timeout;
use std::collections::HashMap;
use std::rc::Rc;
use yew::{hook, use_context, use_state, UseStateHandle};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Kind {
    Error,
    Warning,
    Info,
    Success,
}

pub trait Toast: ToString {
    fn kind(&self) -> Kind;
    fn leave(&self) -> bool;
}

#[derive(derive_more::Display)]
#[display(fmt = "You are not allowed to view this page")]
pub struct Forbidden;

impl Toast for Forbidden {
    fn kind(&self) -> Kind {
        Kind::Warning
    }
    fn leave(&self) -> bool {
        true
    }
}

impl Toast for api::Error {
    fn kind(&self) -> Kind {
        Kind::Error
    }
    fn leave(&self) -> bool {
        false
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "{}", message)]
pub struct Generic {
    pub message: String,
    pub leave: bool,
    pub kind: Kind,
}

impl Toast for Generic {
    fn kind(&self) -> Kind {
        self.kind
    }
    fn leave(&self) -> bool {
        self.leave
    }
}

impl Generic {
    pub fn error(message: impl ToString, leave: bool) -> Self {
        Self { message: message.to_string(), leave, kind: Kind::Error }
    }
    pub fn warning(message: impl ToString, leave: bool) -> Self {
        Self { message: message.to_string(), leave, kind: Kind::Warning }
    }
    pub fn info(message: impl ToString, leave: bool) -> Self {
        Self { message: message.to_string(), leave, kind: Kind::Info }
    }
    pub fn success(message: impl ToString, leave: bool) -> Self {
        Self { message: message.to_string(), leave, kind: Kind::Success }
    }
}

#[derive(Default, Clone)]
pub struct Toasts(pub HashMap<u64, Rc<(Box<dyn Toast>, Timeout)>>);

impl PartialEq for Toasts {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
    }
}

#[derive(Clone, PartialEq)]
pub struct UseToastManagerHandle {
    state: UseStateHandle<Rc<Toasts>>,
    counter: UseStateHandle<u64>,
}

impl UseToastManagerHandle {
    pub fn add(&self, toast: impl Toast + 'static) {
        let mut new = (**self.state).clone();

        let id = *self.counter;
        self.counter.set(id + 1);

        let cloned = self.clone();
        let timer = Timeout::new(4_000, move || cloned.remove(id));

        new.0.insert(id, Rc::new((Box::new(toast), timer)));
        self.state.set(Rc::new(new));
    }

    pub fn remove(&self, id: u64) {
        let mut new = (**self.state).clone();
        new.0.remove(&id);
        self.state.set(Rc::new(new));
    }

    pub fn data(&self) -> Rc<Toasts> {
        Rc::clone(&*self.state)
    }
}

#[derive(Clone, PartialEq)]
pub struct UseToastHandle {
    manager: UseToastManagerHandle,
}

impl UseToastHandle {
    pub fn maybe<V, T: Toast + 'static>(&self, result: Result<V, T>) -> Option<V> {
        result.map_err(|toast| self.add(toast)).ok()
    }

    pub fn add(&self, toast: impl Toast + 'static) {
        self.manager.add(toast)
    }
}

#[hook]
pub fn use_toast() -> UseToastHandle {
    let manager = use_context::<UseToastManagerHandle>().expect("toasts context must be defined");
    UseToastHandle { manager }
}

#[hook]
pub fn use_toast_manager() -> UseToastManagerHandle {
    let state = use_state(|| Rc::new(Toasts::default()));
    let counter = use_state(|| 0);
    UseToastManagerHandle { state, counter }
}
