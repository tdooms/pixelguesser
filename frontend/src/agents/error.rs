use crate::shared::Error;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

pub struct ErrorAgent {
    link: AgentLink<Self>,
    error: Option<Rc<Error>>,
    subscribers: HashSet<HandlerId>,
}

impl Agent for ErrorAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = Error;
    type Output = Rc<Error>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link, error: None, subscribers: HashSet::new() }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        let error = Rc::new(msg);
        for id in &self.subscribers {
            self.link.respond(*id, error.clone());
        }
        self.error = Some(error)
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
