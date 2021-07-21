use crate::agents::WebSocketAgent;
use crate::notifications::{Error, Notification, Warning};
use api::Response;
use std::collections::HashSet;
use std::rc::Rc;
use yew::agent::{Agent, AgentLink, Context, HandlerId};
use yew::{Bridge, Bridged};

pub struct AlertAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    _ws_agent: Box<dyn Bridge<WebSocketAgent>>,
}

impl Agent for AlertAgent {
    type Reach = Context<Self>;
    type Message = Response;
    type Input = Notification;
    type Output = Rc<Notification>;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            _ws_agent: WebSocketAgent::bridge(link.callback(|x| x)),
            link,
            subscribers: Default::default(),
        }
    }

    fn update(&mut self, response: Self::Message) {
        let notification = match response {
            Response::Alert(_, api::Alert::SessionStopped) => {
                Some(Notification::Warning(Warning::SessionStopped))
            }
            Response::Error(api::Error::SessionDoesNotExist(_)) => {
                Some(Notification::Warning(Warning::SessionInvalid))
            }
            Response::Error(api::Error::FaultyRequest) => {
                Some(Notification::Error(Error::FaultyRequest))
            }
            Response::Error(api::Error::InternalServerError) => {
                Some(Notification::Error(Error::InternalServerError))
            }
            Response::Error(api::Error::SessionCreationFailed) => {
                Some(Notification::Error(Error::SessionCreationFailed))
            }
            Response::Error(api::Error::PlayerDoesNotExist(_)) => {
                Some(Notification::Error(Error::PlayerDoesNotExist))
            }
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
