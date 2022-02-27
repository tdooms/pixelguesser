use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use yew::Callback;

use crate::{async_callback, EmitError, Error};
use api::{
    create_quiz, delete_quiz, full_quiz, save_rounds, update_quiz, DraftQuiz, DraftRound,
    FullDraftQuiz, User,
};
use std::borrow::BorrowMut;

#[derive(Clone, Copy)]
pub enum CreateStage {
    Quiz,
    Rounds,
    Summary,
}

struct Inner {
    id: Option<u64>,
    full: FullDraftQuiz,
    stage: CreateStage,

    user: Option<User>,
    onerror: Callback<Error>,
}

#[derive(Clone)]
pub struct CreateState {
    inner: Rc<RefCell<Inner>>,
}

impl Inner {
    pub async fn new(id: Option<u64>, user: Option<User>, onerror: Callback<Error>) -> Self {
        let cloned = user.clone();
        let callback = onerror.reform(Error::Api);

        let full = match id {
            Some(id) => full_quiz(cloned, id).await.emit(&callback).map(Into::into),
            None => None,
        };

        let stage = CreateStage::Quiz;
        Self { user, onerror, full: full.unwrap_or_default(), id, stage }
    }

    async fn set_quiz(&mut self, quiz: DraftQuiz) {
        let callback = self.onerror.reform(Error::Api);

        let new = match &self.id {
            None => create_quiz(self.user.clone(), quiz).await,
            Some(id) => update_quiz(self.user.clone(), *id, quiz).await,
        };

        if let Some(new) = new.emit(&callback) {
            self.full.quiz = new.unwrap().into();
        }
    }

    async fn set_rounds(&mut self, mut rounds: Vec<DraftRound>) {
        let callback = self.onerror.reform(Error::Api);

        for round in &mut rounds {
            round.image.as_mut().map(|x| x.upload());
        }

        if let Some(id) = self.id {
            // TODO: stuff
            save_rounds(self.user.clone(), id, rounds).await.emit(&callback);
        }
    }

    async fn delete(&mut self) {
        let callback = self.onerror.reform(Error::Api);

        if let Some(id) = self.id {
            delete_quiz(self.user.clone(), id).await.emit(&callback);
            self.full = FullDraftQuiz::default();
            self.id = None;
        }
    }
}

impl CreateState {
    pub async fn new(user: Option<User>, id: Option<u64>, onerror: Callback<Error>) -> Self {
        Self { inner: Rc::new(RefCell::new(Inner::new(id, user, onerror).await)) }
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        (*self.inner).borrow().full.rounds.clone()
    }

    pub fn quiz(&self) -> DraftQuiz {
        (*self.inner).borrow().full.quiz.clone()
    }

    pub fn stage(&self) -> CreateStage {
        (*self.inner).borrow().stage
    }

    pub fn onstage(&self, stage: CreateStage) -> Callback<()> {
        let cloned = Rc::clone(&self.inner);
        Callback::from(move |_| (*cloned).borrow_mut().stage = stage)
    }

    pub fn onsetquiz(&self) -> Callback<DraftQuiz> {
        let cloned = Rc::clone(&self.inner);
        Callback::from(move |quiz| {
            let inner = cloned.clone();
            let fut = async move { inner.deref().borrow_mut().set_quiz(quiz).await };
            async_callback(fut, Callback::noop())
        })
    }

    pub fn onsetrounds(&self) -> Callback<Vec<DraftRound>> {
        let cloned = Rc::clone(&self.inner);
        Callback::from(move |rounds| {
            let inner = cloned.clone();
            let fut = async move { inner.deref().borrow_mut().set_rounds(rounds).await };
            async_callback(fut, Callback::noop())
        })
    }

    pub fn ondelete(&self) -> Callback<()> {
        let cloned = Rc::clone(&self.inner);
        Callback::from(move |_| {
            let inner = cloned.clone();
            let fut = async move { inner.deref().borrow_mut().delete().await };
            async_callback(fut, Callback::noop())
        })
    }
}
