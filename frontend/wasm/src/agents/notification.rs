use yew_agent::{Agent, AgentLink, HandlerId, Context};

pub struct NotificationAgent {
    link: AgentLink<Self>,
}

impl Agent for NotificationAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ();
    type Output = ();

    fn create(link: AgentLink<Self>) -> Self {
        Self{ link }
    }

    fn update(&mut self, msg: Self::Message) {

    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {

    }
}