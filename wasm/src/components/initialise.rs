use crate::agents::WebSocketAgent;
use api::{Post, Request};

use yew::agent::Dispatcher;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

#[derive(Clone, Properties)]
pub struct Props {
    pub session_id: u64,
}

pub struct Initialise {
    link: ComponentLink<Self>,
    ws_agent: Dispatcher<WebSocketAgent>,
    props: Props,
    element: NodeRef,
}

impl Component for Initialise {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            ws_agent: WebSocketAgent::dispatcher(),
            link,
            props,
            element: NodeRef::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        match self.element.cast::<HtmlInputElement>() {
            Some(input) => {
                let (name, session_id) = (input.value(), self.props.session_id);
                input.set_value("");

                let request = Request::Post(Post::AddPlayer { session_id, name });
                self.ws_agent.send(request);
                true
            }
            None => {
                // TODO: send to logger
                log::error!("reference not of type input");
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <div class="field is-grouped">
                    <p class="control is-expanded">
                        <input class="input is-large" type="text" placeholder="eg. Alex" ref=self.element.clone()/>
                    </p>
                    <p class="control">
                        <a class="button is-large is-info" onclick=self.link.callback(|_|())>
                            <span class="icon"><i class="fas fa-plus"></i></span>
                        </a>
                    </p>
                </div>
            </>
        }
    }
}
