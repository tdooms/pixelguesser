use std::collections::HashSet;
use std::rc::Rc;

use yew_agent::{Agent, AgentLink, Bridge, Bridged, Context, HandlerId};

use api::Response;

use crate::agents::WebSocketAgent;
use crate::structs::Error;

pub struct ErrorAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Agent for ErrorAgent {
    type Reach = Context<Self>;
    type Message = Response;
    type Input = Error;
    type Output = Rc<Error>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            _ws_agent: WebSocketAgent::bridge(link.callback(|x| x)),
            link,
            subscribers: Default::default(),
        }
    }

    fn update(&mut self, response: Self::Message) {
        let notification = match response {
            Response::Error(api::Error::FaultyRequest) => Some(Error::FaultyRequest),
            Response::Error(api::Error::InternalServerError) => Some(Error::InternalServerError),
            Response::Error(api::Error::SessionCreationFailed) => {
                Some(Error::SessionCreationFailed)
            }
            Response::Error(api::Error::PlayerDoesNotExist(_)) => Some(Error::PlayerDoesNotExist),
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
