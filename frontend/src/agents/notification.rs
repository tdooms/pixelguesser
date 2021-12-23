use yew_agent::{Agent, AgentLink, Context, HandlerId};

pub struct NotificationAgent {
    _link: AgentLink<Self>,
}

impl Agent for NotificationAgent {
    type Reach = Context<Self>;
    type Message = ();
    type Input = ();
    type Output = ();

    fn create(_link: AgentLink<Self>) -> Self {
        Self { _link }
    }

    fn update(&mut self, _: Self::Message) {}

    fn handle_input(&mut self, _: Self::Input, _: HandlerId) {}
}
