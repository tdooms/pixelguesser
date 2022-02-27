use cobul::props::ColumnSize;
use cobul::{Column, Columns, Container, Loading, Section};
// use futures::prelude::*;
use yew::prelude::*;
// use yew_router::prelude::{History, RouterScopeExt};
//
// use crate::{Auth, Route};
// use api::{quizzes, Quiz, IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
// use components::{MainNavbar, QuizCard};
//
// pub struct Overview {
//     quizzes: Option<Vec<Quiz>>,
// }
//
// #[derive(Debug)]
// pub enum Msg {
//     Quizzes(Result<Vec<Quiz>, api::Error>),
// }
//
// impl Component for Overview {
//     type Message = Msg;
//     type Properties = ();
//
//     fn create(ctx: &Context<Self>) -> Self {
//         let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
//
//         ctx.link().send_future(quizzes(auth.into()).map(Msg::Quizzes));
//         Self { quizzes: None }
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         ctx.link().history().unwrap().replace(Route::Overview);
//
//         match msg {
//             Msg::Quizzes(Ok(quizzes)) => {
//                 self.quizzes = Some(quizzes);
//                 true
//             }
//             Msg::Quizzes(Err(err)) => {
//                 log::error!("http error: {:?}", err);
//                 false
//             }
//         }
//     }
//
//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         let view_quiz_card = |quiz: &Quiz| {
//             let Quiz { title, description, image, id, creator, public, .. } = quiz.clone();
//
//             let image = image
//                 .as_ref()
//                 .map(|path| format!("{}/{}", IMAGE_ENDPOINT, path))
//                 .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());
//
//             html! {
//                 <Column size={ColumnSize::Is3}>
//                     <QuizCard {id} {title} {description} {image} {creator} {public}/>
//                 </Column>
//             }
//         };
//
//         let view_quizzes = |quizzes: &Vec<Quiz>| {
//             html! {
//                 <>
//                 <MainNavbar/>
//                 <Section>
//                     <Container>
//                     <Columns multiline=true>
//                         { for quizzes.iter().map(view_quiz_card) }
//                     </Columns>
//                     </Container>
//                 </Section>
//                 </>
//             }
//         };
//
//         match &self.quizzes {
//             Some(vec) => view_quizzes(vec),
//             None => html! { <Loading /> },
//         }
//     }
// }

use crate::loader::Msg::Quiz;
use api::{IMAGE_ENDPOINT, IMAGE_PLACEHOLDER};
use shared::Auth;
use yew::HtmlResult;

#[function_component(Overview)]
pub fn overview() -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user().ok();
    let quizzes = use_quizzes(user)?;

    let view_quiz_card = |quiz: &Quiz| {
        let Quiz { title, description, image, id, creator, public, .. } = quiz.clone();

        let image = image
            .as_ref()
            .map(|path| format!("{}/{}", IMAGE_ENDPOINT, path))
            .unwrap_or_else(|| IMAGE_PLACEHOLDER.to_owned());

        html! {
            <Column size={ColumnSize::Is3}>
                <QuizCard {id} {title} {description} {image} {creator} {public}/>
            </Column>
        }
    };

    html! {
        <>
        <MainNavbar/>
        <Section>
            <Container>
            <Columns multiline=true>
                { for quizzes.iter().map(view_quiz_card) }
            </Columns>
            </Container>
        </Section>
        </>
    }
}
