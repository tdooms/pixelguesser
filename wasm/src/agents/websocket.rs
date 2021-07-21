use crate::agents::AlertAgent;
use crate::globals::API_ENDPOINT;
use crate::notifications::{Error, Notification, Warning};
use api::{Request, Response};
use std::collections::HashSet;
use yew::agent::Dispatcher;
use yew::prelude::*;
use yew::worker::{Agent, AgentLink, Context, HandlerId};
use yew_services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub enum Msg {
    Response(Response),
    Notify(Notification),
    Connected,
    Disconnected,
}

pub struct WebSocketAgent {
    link: AgentLink<Self>,
    ws: WebSocketTask,

    buffer: Vec<Request>,
    subscribers: HashSet<HandlerId>,
    logger: Dispatcher<AlertAgent>,

    connected: bool,
}

impl WebSocketAgent {
    fn send(&mut self, request: Request) {
        match serde_json::to_string(&request) {
            Ok(string) => self.ws.send(Ok(string)),
            Err(err) => self.logger.send(Notification::Error(Error::JsonError(err))),
        }
    }

    fn connect(link: &AgentLink<WebSocketAgent>) -> WebSocketTask {
        let url = format!("ws://{}/ws", API_ENDPOINT);

        log::debug!("connecting to {}", url);

        let mapper = |result: anyhow::Result<String>| match result {
            Ok(string) => match serde_json::from_str(&string) {
                Ok(response) => Msg::Response(response),
                Err(err) => Msg::Notify(Notification::Error(Error::JsonError(err))),
            },
            Err(err) => Msg::Notify(Notification::Error(Error::WsError(err))),
        };

        let callback = link.callback(mapper);

        let notification = link.callback(|status| match status {
            WebSocketStatus::Opened => Msg::Connected,
            WebSocketStatus::Closed | WebSocketStatus::Error => Msg::Disconnected,
        });

        match WebSocketService::connect_text(&url, callback, notification) {
            Ok(task) => task,
            Err(err) => {
                log::warn!("{:?}", err);
                Self::connect(link)
            }
        }
    }
}

impl Agent for WebSocketAgent {
    type Reach = Context<Self>;
    type Message = Msg;
    type Input = Request;
    type Output = Response;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            ws: Self::connect(&link),
            link,
            buffer: vec![],
            subscribers: HashSet::new(),
            logger: AlertAgent::dispatcher(),
            connected: false,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::Connected => {
                log::debug!("websocket connected");

                self.connected = true;
                let buffer = std::mem::take(&mut self.buffer);

                for request in buffer {
                    self.send(request);
                }
            }
            Msg::Disconnected => {
                let notification = Notification::Warning(Warning::WsDisconnect);
                self.logger.send(notification);

                self.connected = false;
                self.ws = Self::connect(&self.link);
            }
            Msg::Response(response) => {
                log::debug!("websocket response: {:?}", response);

                for sub in self.subscribers.iter() {
                    self.link.respond(*sub, response.clone());
                }
            }
            Msg::Notify(_) => {} // self.logger.send(notification),
        }
    }

    fn connected(&mut self, id: worker::HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, request: Self::Input, _: worker::HandlerId) {
        log::debug!("websocket request: {:?}", request);

        match self.connected {
            true => self.send(request),
            false => self.buffer.push(request),
        }
    }

    fn disconnected(&mut self, id: worker::HandlerId) {
        self.subscribers.remove(&id);
    }
}
