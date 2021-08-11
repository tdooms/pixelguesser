use std::collections::HashMap;

use yew::prelude::*;

use api::{Alert, Post, Reply, Request, Response, Session, Stage};

use crate::agents::WebSocketAgent;
use crate::pages::host::InnerHost;
use shared::Session;
use yew::utils::NeqAssign;

pub enum Msg {
    Created((u64, Session)),
    Hosted(),
    Revealed,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HostLoaderProps {
    pub quiz_id: i64,
}

pub struct Host {
    props: HostLoaderProps,
    link: ComponentLink<Self>,

    session: Option<(u64, Session)>,
}

impl Component for Host {
    type Message = Msg;
    type Properties = HostLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: create session
        // TODO: host session

        Self { props, link, session: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Created(tuple) => {
                self.session = Some(tuple);
                true
            },
            Msg::Hosted() => {
                // TODO: initialise ws socket
                false
            },
            Msg::Revealed => {
                // TODO: change stage of session
                true
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.session {
            Some((session_id, session)) => {
                html! {<InnerHost session={session.clone()} session_id={*session_id}/>}
            }
            None => html! {},
        }
    }
}
