use yew::prelude::*;
use yew::utils::NeqAssign;

use shared::Session;

use crate::pages::manage::InnerManage;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ManageLoaderProps {
    pub session_id: u64,
}

pub enum Msg {
    Updated(Session),
    Managed(Session),
}

pub struct Manage {
    props: ManageLoaderProps,
    session: Option<Session>,
}

impl Component for Manage {
    type Message = Msg;
    type Properties = ManageLoaderProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: manage session
        Self { props, session: None }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Updated(session) => {
                // TODO: send websocket session change
                false
            }
            Msg::Managed(session) => {
                self.session = Some(session);
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        match &self.session {
            Some(session) => {
                let onchange = self.link.callback(Msg::Updated);
                html! {<InnerManage session={session.clone()} session_id={self.props.session_id} onchange={onchange} />}
            }
            None => html! {},
        }
    }
}
