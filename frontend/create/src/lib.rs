mod center_space;
mod create_quiz;
mod create_rounds;
mod quiz_form;
mod round_form;
mod round_list;
mod summary;

use crate::create_quiz::CreateQuiz;
use crate::create_rounds::CreateRounds;
use crate::summary::Summary;

use std::ops::Deref;

use cobul::Loading;
use futures::FutureExt;
use yew::prelude::*;
use yew_agent::{Dispatched, Dispatcher};
use yew_router::prelude::*;

use agents::{Auth, ErrorAgent};
use shared::{Error, Route};

use api::{DraftQuiz, DraftRound, FullQuiz, Quiz};

#[derive(Debug)]
pub enum Msg {
    SubmitQuiz(DraftQuiz),
    DeleteQuiz,
    SaveRounds(Vec<DraftRound>),
    ChangeStage(Stage),

    QuizLoaded(Result<FullQuiz, api::Error>),
    QuizCreated(Result<Quiz, api::Error>),
    QuizUpdated(Result<Quiz, api::Error>),
    QuizDeleted(Result<(), api::Error>),

    RoundsSaved(Result<Vec<DraftRound>, api::Error>),
}

#[derive(Debug)]
pub enum Stage {
    Load,
    Quiz,
    Rounds,
    Summary,
    Leave,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: Option<u64>,
}

pub struct Create {
    stage: Stage,
    errors: Dispatcher<ErrorAgent>,

    // If id is Some, it means the quiz exists in the database
    id: Option<u64>,
    quiz: Option<DraftQuiz>,
    rounds: Vec<DraftRound>,
}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let stage = match ctx.props().id {
            None => Stage::Quiz,
            Some(id) => {
                let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();
                ctx.link().send_future(api::full_quiz(auth.into(), id).map(Msg::QuizLoaded));
                Stage::Load
            }
        };
        let errors = ErrorAgent::dispatcher();
        Self { stage, quiz: None, id: None, rounds: vec![], errors }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();
        let (auth, _) = ctx.link().context::<Auth>(Callback::noop()).unwrap();

        match msg {
            Msg::SubmitQuiz(quiz) => {
                match (ctx.props().id, &self.quiz) {
                    (Some(_), Some(old)) if old.deref() == &quiz => {} // Don't resubmit
                    (None, _) => link.send_future(
                        api::create_quiz(auth.into(), quiz.clone()).map(Msg::QuizCreated),
                    ),
                    (Some(id), _) => link.send_future(
                        api::update_quiz(auth.into(), id, quiz.clone()).map(Msg::QuizUpdated),
                    ),
                }

                self.quiz = Some(quiz);
                self.stage = Stage::Rounds;
            }
            Msg::SaveRounds(rounds) => {
                let id = ctx.props().id.or(self.id);

                match (id, rounds == *self.rounds) {
                    (Some(id), false) => link.send_future(
                        api::save_rounds(auth.into(), id, rounds.clone()).map(Msg::RoundsSaved),
                    ),
                    (Some(_), true) => {}
                    (None, _) => self.errors.send(Error::UnlinkedRounds),
                }

                self.rounds = rounds.into_iter().map(Into::into).collect();
            }
            Msg::QuizCreated(Ok(quiz)) => {
                self.id = Some(quiz.id);
                self.quiz = Some(quiz.into())
            }
            Msg::DeleteQuiz => match ctx.props().id {
                Some(id) => {
                    link.send_future(api::delete_quiz(auth.into(), id).map(Msg::QuizDeleted))
                }
                None => self.errors.send(Error::DeleteUncommittedQuiz),
            },
            Msg::QuizLoaded(Ok(quiz)) => {
                self.stage = Stage::Quiz;
                self.rounds = quiz.rounds.iter().cloned().map(Into::into).collect();
                self.quiz = Some(quiz.into().into());
            }
            Msg::RoundsSaved(Ok(rounds)) => self.rounds = rounds,
            Msg::QuizUpdated(Ok(_)) => {}
            Msg::QuizDeleted(Ok(_)) => link.history().unwrap().push(Route::Overview),
            Msg::ChangeStage(Stage::Leave) => link.history().unwrap().push(Route::Overview),
            Msg::ChangeStage(Stage::Load) => log::error!("cannot change stage to load"),
            Msg::ChangeStage(stage) => self.stage = stage,
            Msg::RoundsSaved(Err(err))
            | Msg::QuizCreated(Err(err))
            | Msg::QuizUpdated(Err(err))
            | Msg::QuizDeleted(Err(err))
            | Msg::QuizLoaded(Err(err)) => {
                self.errors.send(err);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        match self.stage {
            Stage::Quiz => {
                let quiz = self.quiz.clone().map(Into::into);
                let onsubmit = link.callback(Msg::SubmitQuiz);
                let oncancel = link.callback(|_| Msg::ChangeStage(Stage::Leave));
                let ondelete = link.callback(|_| Msg::DeleteQuiz);
                html! { <CreateQuiz {quiz} {onsubmit} {oncancel} {ondelete}/> }
            }
            Stage::Rounds => {
                let rounds: Vec<DraftRound> = self.rounds.iter().cloned().map(Into::into).collect();
                let ondone = link.callback(|_| Msg::ChangeStage(Stage::Summary));
                let onback = link.callback(|_| Msg::ChangeStage(Stage::Quiz));
                let onsave = link.callback(Msg::SaveRounds);
                html! { <CreateRounds {rounds} {ondone} {onback} {onsave}/> }
            }
            Stage::Summary => {
                let onback = link.callback(|_| Msg::ChangeStage(Stage::Rounds));
                let onfinish = link.callback(|_| Msg::ChangeStage(Stage::Leave));
                let quiz = self.quiz.clone().unwrap();
                html! { <Summary rounds={self.rounds.clone()} {quiz} {onback} {onfinish}/>}
            }
            Stage::Load => html! { <Loading/> },
            Stage::Leave => html! {},
        }
    }
}
