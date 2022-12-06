use std::collections::HashMap;
use std::rc::Rc;

use gloo::timers::callback::Timeout;
use yew::{hook, use_context, use_state, UseStateHandle};

#[derive(Clone, Debug, Copy, PartialEq, Default)]
pub enum Level {
    Error,
    Warning,
    #[default]
    Info,
    Success,
}

pub trait Toast {
    fn level(&self) -> Level;
    fn leave(&self) -> bool;

    fn message(&self) -> String;
    fn help(&self) -> String;
    fn reason(&self) -> String;
}

pub struct Forbidden(pub &'static str);

impl Toast for Forbidden {
    fn level(&self) -> Level {
        Level::Warning
    }
    fn leave(&self) -> bool {
        true
    }
    fn message(&self) -> String {
        "Access Denied".to_owned()
    }
    fn help(&self) -> String {
        "You are not allowed to view this page".to_owned()
    }
    fn reason(&self) -> String {
        format!("Need to be at least {}", self.0)
    }
}

#[derive(Default)]
struct Generic {
    pub message: String,
    pub help: String,
    pub reason: String,

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
    fn message(&self) -> String {
        self.message.clone()
    }
    fn help(&self) -> String {
        self.help.clone()
    }
    fn reason(&self) -> String {
        self.reason.clone()
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

        match toast.level() {
            Level::Error => tracing::error!("{}: {}", toast.message(), toast.reason()),
            Level::Warning => tracing::warn!("{}: {}", toast.message(), toast.reason()),
            Level::Info => tracing::info!("{}: {}", toast.message(), toast.reason()),
            Level::Success => tracing::debug!("{}: {}", toast.message(), toast.reason()),
        }

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

    pub fn api<V>(&self, result: Result<V, api::Error>) -> Option<V> {
        if result.is_ok() {
            return result.ok();
        }

        let error = match result.err().unwrap() {
            api::Error::Hasura(e) => Generic {
                message: "Hasura Error".to_owned(),
                help: "An error occurred while communicating with the server".to_owned(),
                reason: e.to_string(),
                kind: Level::Error,
                leave: true,
            },
            // Err(api::Error::Serde(e)) => {
            // }
            // api::Error::Session(e) => Generic {
            //     message: "Session Error".to_owned(),
            //     help: "An error occurred while communicating with the server".to_owned(),
            //     reason: e.to_string(),
            //     kind: Level::Error,
            //     leave: true,
            // },
            api::Error::Unreachable(host, url) => Generic {
                message: format!("{host} Server Unreachable"),
                help: "Please let us know if the issue persists".to_owned(),
                reason: format!("Endpoint {url} is not reachable"),
                kind: Level::Error,
                leave: true,
            },
            api::Error::ErrorStatus(host) => Generic {
                message: format!("{host} returning an error status"),
                help: "Please let us know if the issue persists".to_owned(),
                reason: "".to_owned(),
                kind: Level::Error,
                leave: true,
            },
            api::Error::InvalidResponse(host, e) => Generic {
                message: "Invalid Response".to_owned(),
                help: format!("The {host} server returned an invalid response"),
                reason: e.to_string(),
                kind: Level::Error,
                leave: true,
            },
            api::Error::WsClosed => Generic {
                message: "Websocket closed".to_owned(),
                help: "The server closed the websocket connection".to_owned(),
                reason: "".to_owned(),
                kind: Level::Error,
                leave: true,
            },
            api::Error::WsFailure => Generic {
                message: "Websocket failure".to_owned(),
                help: "The websocket connection failed for an unknown reason".to_owned(),
                reason: "".to_owned(),
                kind: Level::Error,
                leave: true,
            },
            api::Error::WsBytes => Generic {
                message: "Invalid websocket message".to_owned(),
                help: "The websocket connection received bytes".to_owned(),
                reason: "This is not supported and shouldn't happen".to_owned(),
                kind: Level::Warning,
                leave: true,
            },
            api::Error::EmptyResponse => Generic {
                message: "Hasura error".to_owned(),
                help: "The graphql server returned an empty response".to_owned(),
                reason: "".to_owned(),
                kind: Level::Error,
                leave: true,
            },
        };

        self.add(error);
        None
    }

    pub fn add(&self, toast: impl Toast + 'static) {
        self.manager.add(toast)
    }

    pub fn custom(&self, message: impl ToString, leave: bool, kind: Level) {
        self.add(Generic { message: message.to_string(), kind, leave, ..Default::default() })
    }
    pub fn error(&self, message: impl ToString, leave: bool) {
        self.custom(message, leave, Level::Error);
    }
    pub fn warning(&self, message: impl ToString, leave: bool) {
        self.custom(message, leave, Level::Warning);
    }
    pub fn info(&self, message: impl ToString, leave: bool) {
        self.custom(message, leave, Level::Info);
    }
    pub fn success(&self, message: impl ToString, leave: bool) {
        self.custom(message, leave, Level::Success);
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
