use std::collections::HashMap;
use shared::Session;
use yew::utils::NeqAssign;
use yew::prelude::*;

use crate::pages::host::InnerHost;


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
            }
            Msg::Hosted() => {
                // TODO: initialise ws socket
                false
            }
            Msg::Revealed => {
                // TODO: change stage of session
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.session {
            Some((_, session)) => {
                let onrevealed = self.link.callback(|_| Msg::Revealed);
                html! {<InnerHost session={session.clone()} onrevealed={onrevealed} quiz={self.props.}/> }
            }
            None => html! {},
        }
    }
}
