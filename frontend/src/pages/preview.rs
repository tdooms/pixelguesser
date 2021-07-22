use yew::prelude::*;

use api::Status;

use crate::host::Pixelate;

pub enum Msg {}

pub struct Preview {
    link: ComponentLink<Self>,
}

impl Component for Preview {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! { <Pixelate on_revealed={self.link.callback(|_|())} url={"banana.jpg"} status={Status::Playing{paused: false}} /> }
    }
}
