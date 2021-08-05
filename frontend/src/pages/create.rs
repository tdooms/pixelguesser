use crate::create::CreateQuiz;
use crate::route::Route;

use gloo_file::File;
use yew::prelude::*;

pub enum Msg {
    Continue(File),
    Cancel,
}

#[derive(Properties, Clone)]
pub struct Props {}

pub struct Create {}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Continue(file) => file.read_as_bytes,
            Msg::Cancel => yew_router::push_route(Route::Overview),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! { <CreateQuiz /> }
    }
}
