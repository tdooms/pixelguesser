// use std::ops::Deref;
//
// use cobul::Loading;
// use futures::FutureExt;
// use yew::prelude::*;
// use yew_agent::{Dispatched, Dispatcher};
// use yew_router::prelude::*;
//
// use agents::{Auth, ErrorAgent};
// use api::{DraftQuiz, DraftRound, FullQuiz, ManagedQuiz, Quiz};
// use shared::{Error, Route};
//
// use crate::create_quiz::CreateQuiz;
// use crate::create_rounds::CreateRounds;
// use crate::summary::Summary;
//
// mod center_space;
// mod create_quiz;
// mod create_rounds;
// mod quiz_form;
// mod round_form;
// mod round_list;
// mod state;
// mod summary;
//
// #[derive(Debug)]
// pub enum Msg {
//     ChangeQuiz(DraftQuiz),
//     DeleteQuiz,
//     SaveRounds(Vec<DraftRound>),
//
//     QuizChanged,
//     QuizDeleted,
//     RoundsSaved,
//
//     ChangeStage(Stage),
//     Error(api::Error),
// }
//
// #[derive(Debug)]
// pub enum Stage {
//     Load,
//     Quiz(ManagedQuiz),
//     Rounds(ManagedQuiz),
//     Summary(ManagedQuiz),
//     Leave,
// }
//
// #[derive(Properties, Clone, PartialEq)]
// pub struct Props {
//     pub id: Option<u64>,
// }
//
// pub struct Create {
//     stage: Stage,
//     errors: Dispatcher<ErrorAgent>,
//
//     quiz: Option<ManagedQuiz>,
// }
//
// impl Component for Create {
//     type Message = Msg;
//     type Properties = Props;
//
//     fn create(ctx: &Context<Self>) -> Self {
//         let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
//         let onerror = ctx.link().callback(Msg::Error);
//
//         let fut = ManagedQuiz::new(auth.into(), onerror, ctx.props().id);
//         ctx.link().send_future(fut.map(|x| Msg::ChangeStage(Stage::Quiz(x))));
//
//         let errors = ErrorAgent::dispatcher();
//         Self { stage: Stage::Load, quiz: None, errors }
//     }
//
//     fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
//         let link = ctx.link();
//         let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
//
//         match (msg, &self.stage) {
//             (Msg::ChangeQuiz(quiz), Stage::Quiz(managed)) => {
//                 link.send_future(self.quiz.unwrap().change_quiz(quiz).map(|_| Msg::QuizChanged));
//                 self.stage = Stage::Rounds(managed.clone());
//             }
//             (Msg::SaveRounds(rounds), Stage::Rounds(_)) => {
//                 let fut = self.quiz.unwrap().update_rounds(rounds);
//                 link.send_future(fut.map(|_| Msg::RoundsSaved));
//             }
//             (Msg::DeleteQuiz, Stage::Quiz(managed)) => {
//                 link.send_future(managed.clone().delete_quiz().map(|_| Msg::QuizDeleted));
//             }
//             (Msg::QuizDeleted, _) => link.history().unwrap().push(Route::Overview),
//             (Msg::ChangeStage(Stage::Leave), _) => link.history().unwrap().push(Route::Overview),
//             (Msg::ChangeStage(Stage::Load), _) => log::error!("cannot change stage to load"),
//             (Msg::ChangeStage(stage), _) => self.stage = stage,
//             (Msg::Error(error), _) => self.errors.send(Error::Api(error)),
//             (Msg::QuizChanged, _) => log::info!("quiz changed"),
//             (Msg::RoundsSaved, _) => log::info!("rounds saved"),
//             _ => {}
//         }
//         true
//     }
//
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let link = ctx.link();
//
//         match &self.stage {
//             Stage::Quiz(managed) => {
//                 let quiz = managed.quiz();
//                 let onsubmit = link.callback(Msg::ChangeQuiz);
//                 let oncancel = link.callback(|_| Msg::ChangeStage(Stage::Leave));
//                 let ondelete = link.callback(|_| Msg::DeleteQuiz);
//                 html! { <CreateQuiz {quiz} {onsubmit} {oncancel} {ondelete}/> }
//             }
//             Stage::Rounds(managed) => {
//                 let rounds = managed.rounds();
//                 let ondone = link.callback(|_| Msg::ChangeStage(Stage::Summary(managed.clone())));
//                 let onback = link.callback(|_| Msg::ChangeStage(Stage::Quiz(managed.clone())));
//                 let onsave = link.callback(Msg::SaveRounds);
//                 html! { <CreateRounds {rounds} {ondone} {onback} {onsave}/> }
//             }
//             Stage::Summary(managed) => {
//                 let onback = link.callback(|_| Msg::ChangeStage(Stage::Rounds(managed.clone())));
//                 let onfinish = link.callback(|_| Msg::ChangeStage(Stage::Leave));
//                 let quiz = managed.quiz();
//                 let rounds = managed.rounds();
//                 html! { <Summary {rounds} {quiz} {onback} {onfinish}/>}
//             }
//             Stage::Load => html! { <Loading/> },
//             Stage::Leave => html! {},
//         }
//     }
// }

mod quiz_form;
mod quiz_page;
mod round_edit;
mod round_form;
mod round_list;
mod round_page;
mod round_preview;
mod summary;

use crate::quiz_page::QuizPage;
use crate::round_edit::RoundEdit;
use crate::summary::Summary;

use cobul::Loading;
use futures::FutureExt;
use shared::{use_create_state, Auth, CreateStage, CreateState, Errors, Route};

use yew::prelude::*;
use yew_router::hooks::use_navigator;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<u64>,
}

#[function_component(Create)]
pub fn create(props: &Props) -> HtmlResult {
    let user = use_context::<Auth>().unwrap().user();
    let errors = use_context::<Errors>().unwrap();
    let navigator = use_navigator().unwrap();
    let state = use_create_state(user.ok(), props.id)?;

    let show_state = |state: &CreateState| match state.stage() {
        CreateStage::Quiz => {
            let onsubmit = state.onsetquiz();
            let oncancel = Callback::from(|_| navigator.push(Route::Overview));
            let ondelete = state.ondelete();
            let quiz = state.quiz();

            html! { <CreateQuiz {quiz} {onsubmit} {oncancel} {ondelete}/> }
        }
        CreateStage::Rounds => {
            let onsave = state.onsetrounds();
            let ondone = state.onstage(CreateStage::Summary);
            let onback = state.onstage(CreateStage::Quiz);
            let rounds = state.rounds();

            html! { <CreateRounds {rounds} {onsave} {ondone} {onback}/> }
        }
        CreateStage::Summary => {
            let ondone = Callback::from(|_| navigator.push(Route::Overview));
            let onback = state.onstage(CreateStage::Quiz);
            let (quiz, rounds) = (state.quiz(), state.rounds());

            html! { <Summary {quiz} {rounds} {ondone} {onback}/> }
        }
    };

    match &state {
        None => html! { <Loading /> },
        Some(state) => show_state(state),
    }
}
