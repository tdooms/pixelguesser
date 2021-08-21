// use yew_agent::Dispatcher;
// use yew::prelude::*;
//
// use crate::pages::create::CreateQuiz;
// use crate::route::Route;
// use graphql::{NewQuiz, DraftRound};
//
// pub enum Msg {
//     Continue(NewQuiz),
//     Change(Vec<DraftRound>),
//     Cancel,
// }
//
// #[derive(Properties, Clone)]
// pub struct Props {}
//
// pub struct Create {
//     props: Props,
//     link: ComponentLink<Self>,
// }
//
// impl Component for Create {
//     type Message = Msg;
//     type Properties = Props;
//
//     fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
//         Self { props, link }
//     }
//
//     fn update(&mut self, msg: Self::Message) -> ShouldRender {
//         match msg {
//             Msg::Continue(quiz) => {
//                 // let post = Post::UploadQuiz { quiz };
//                 // self.ws_agent.send(Request::Post(post));
//                 // TODO: send quiz
//             }
//             Msg::Cancel => yew_router::push_route(Route::Overview),
//             Msg::Change(change) => {}
//         }
//         true
//     }
//
//     fn change(&mut self, _props: Self::Properties) -> ShouldRender {
//         false
//     }
//
//     fn view(&self) -> Html {
//         let oncancel = self.link.callback(|_| Msg::Cancel);
//         let oncontinue = self.link.callback(Msg::Continue);
//
//         html! { <CreateQuiz oncontinue={oncontinue} oncancel={oncancel}/> }
//     }
// }
