use std::collections::HashMap;
use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::{hook, use_context, use_state, UseStateHandle};

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Level {
    Error,
    Warning,
    Info,
    Success,
}

impl From<Level> for log::Level {
    fn from(level: Level) -> Self {
        match level {
            Level::Error => log::Level::Error,
            Level::Warning => log::Level::Warn,
            Level::Info => log::Level::Info,
            Level::Success => log::Level::Debug,
        }
    }
}

pub trait Toast: ToString {
    fn level(&self) -> Level;
    fn leave(&self) -> bool;
}

#[derive(derive_more::Display)]
#[display(fmt = "You are not allowed to view this page")]
pub struct Forbidden;

impl Toast for Forbidden {
    fn level(&self) -> Level {
        Level::Warning
    }
    fn leave(&self) -> bool {
        true
    }
}

impl Toast for api::Error {
    fn level(&self) -> Level {
        Level::Error
    }
    fn leave(&self) -> bool {
        false
    }
}

#[derive(derive_more::Display)]
#[display(fmt = "{}", message)]
struct Generic {
    pub message: String,
    pub kind: Level,
    pub leave: bool,
}

impl Toast for Generic {
    fn level(&self) -> Level {
        self.kind
    }

    fn leave(&self) -> bool {
        self.leave
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

        log::log!(toast.level().into(), "{}", toast.to_string());

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

    pub fn error(&self, message: impl ToString, leave: bool) {
        self.manager.add(Generic { message: message.to_string(), leave, kind: Level::Error })
    }
    pub fn warning(&self, message: impl ToString, leave: bool) {
        self.manager.add(Generic { message: message.to_string(), leave, kind: Level::Warning })
    }
    pub fn info(&self, message: impl ToString, leave: bool) {
        self.manager.add(Generic { message: message.to_string(), leave, kind: Level::Info })
    }
    pub fn success(&self, message: impl ToString, leave: bool) {
        self.manager.add(Generic { message: message.to_string(), leave, kind: Level::Success })
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
