use cobul::Loading;
use futures::FutureExt;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::graphql::*;
use crate::shared::{Error, Route, User};

use super::{CreateQuiz, CreateRounds, Summary};

#[derive(Debug)]
pub enum Msg {
    SaveRounds(Vec<DraftRound>),

    RoundsSaved(Result<Vec<DraftRound>, Error>),

    Summary,
    Overview,
    Quiz,
    Rounds,

    SubmitQuiz(DraftQuiz),
    DeleteQuiz,

    QuizLoaded(Result<(Quiz, Vec<Round>), Error>),
    QuizDeleted(Result<Option<u64>, Error>),
    QuizUpdated(Result<(Option<u64>, DraftQuiz), Error>),
    QuizInserted(Result<(Option<u64>, DraftQuiz), Error>),
}

#[derive(Debug)]
enum Stage {
    Load,
    Quiz,
    Rounds,
    Summary,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub quiz_id: Option<u64>,
}

pub struct Create {
    stage: Stage,
    quiz: Option<DraftQuiz>,
    id: Option<u64>,
    rounds: Vec<DraftRound>,
}

impl Component for Create {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let stage = match ctx.props().quiz_id {
            None => Stage::Quiz,
            Some(id) => {
                let user = ctx.link().context::<User>(Callback::noop()).map(|(user, _)| user);
                ctx.link().send_future(quiz(user, id).map(Msg::QuizLoaded));
                Stage::Load
            }
        };
        Self { stage, quiz: None, id: None, rounds: vec![] }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link();

        let user = ctx.link().context::<User>(Callback::noop()).map(|(user, _)| user);

        match msg {
            Msg::SubmitQuiz(quiz) => {
                match (ctx.props().quiz_id, self.quiz.as_ref() == Some(&quiz)) {
                    (None, _) => {
                        link.send_future(insert_quiz(user, quiz.clone()).map(Msg::QuizInserted))
                    }
                    (Some(id), false) => {
                        link.send_future(update_quiz(user, id, quiz.clone()).map(Msg::QuizUpdated))
                    }
                    (Some(id), true) => {} // Don't resubmit
                }

                self.quiz = Some(quiz);
                self.stage = Stage::Rounds;
            }
            Msg::QuizLoaded(Ok((quiz, rounds))) => {
                self.quiz = Some(quiz.into());
                self.rounds = rounds.into_iter().map(Into::into).collect();
                self.stage = Stage::Quiz
            }
            Msg::QuizLoaded(Err(error)) => {
                log::info!("quiz load {}", error)
            }
            Msg::QuizInserted(Err(x)) => {
                log::info!("quiz upload {:?}", x)
            }
            Msg::QuizInserted(Ok((option, quiz))) => {
                self.id = option;
                self.quiz = Some(quiz)
            }
            Msg::Overview => ctx.link().history().unwrap().push(Route::Overview),
            Msg::Quiz => self.stage = Stage::Quiz,
            Msg::Rounds => self.stage = Stage::Rounds,
            Msg::Summary => self.stage = Stage::Summary,
            Msg::SaveRounds(rounds) => {
                let id = ctx.props().quiz_id.or(self.id);
                match (id, rounds == self.rounds) {
                    (Some(id), false) => ctx
                        .link()
                        .send_future(save_rounds(user, id, rounds.clone()).map(Msg::RoundsSaved)),
                    (Some(id), true) => {}
                    (None, _) => {} // TODO: give error
                }
                self.rounds = rounds.into_iter().map(Into::into).collect();
            }

            Msg::DeleteQuiz => {
                match ctx.props().quiz_id {
                    Some(id) => ctx.link().send_future(delete_quiz(user, id).map(Msg::QuizDeleted)),
                    None => {} // TODO: error
                }
            }
            Msg::QuizDeleted(_) => ctx.link().history().unwrap().push(Route::Overview),
            Msg::QuizUpdated(x) => {
                log::info!("quiz update {:?}", x)
            }
            Msg::RoundsSaved(Ok(rounds)) => {
                self.rounds = rounds;
            }
            Msg::RoundsSaved(Err(err)) => {
                log::info!("rounds save {:?}", err)
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        match self.stage {
            Stage::Load => {
                html! { <Loading/> }
            }
            Stage::Quiz => {
                let quiz = self.quiz.clone().map(Into::into);
                let onsubmit = link.callback(Msg::SubmitQuiz);
                let oncancel = link.callback(|_| Msg::Overview);
                let ondelete = link.callback(|_| Msg::DeleteQuiz);
                html! { <CreateQuiz quiz={quiz} onsubmit={onsubmit} oncancel={oncancel} ondelete={ondelete}/> }
            }
            Stage::Rounds => {
                let rounds: Vec<DraftRound> = self.rounds.iter().cloned().map(Into::into).collect();
                let ondone = link.callback(|_| Msg::Summary);
                let onback = link.callback(|_| Msg::Quiz);
                let onsave = link.callback(Msg::SaveRounds);
                html! { <CreateRounds rounds={rounds} ondone={ondone} onback={onback} onsave={onsave}/> }
            }
            Stage::Summary => {
                html! { <Summary rounds={self.rounds.clone()} quiz={self.quiz.clone().unwrap()}
                onback={link.callback(|_| Msg::Rounds)} onfinish={link.callback(|_| Msg::Overview)}/>}
            }
        }
    }
}
