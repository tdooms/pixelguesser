use std::collections::HashSet;
use std::rc::Rc;

use yew_agent::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId};

use api::Response;

use crate::agents::WebSocketAgent;
use crate::structs::Info;

pub struct InfoAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Agent for InfoAgent {
    type Reach = Context<Self>;
    type Message = Response;
    type Input = Info;
    type Output = Rc<Info>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            _ws_agent: WebSocketAgent::bridge(link.callback(|x| x)),
            link,
            subscribers: Default::default(),
        }
    }

    fn update(&mut self, response: Self::Message) {
        let notification = match response {
            Response::Alert(_, api::Alert::SessionStopped) => Some(Info::SessionStopped),
            Response::Error(api::Error::SessionDoesNotExist(_)) => Some(Info::SessionInvalid),
            _ => None,
        };

        if let Some(notification) = notification {
            self.link.send_input(notification)
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, notification: Self::Input, _: HandlerId) {
        let rc = Rc::new(notification);
        for sub in &self.subscribers {
            self.link.respond(*sub, rc.clone())
        }
    }
}
