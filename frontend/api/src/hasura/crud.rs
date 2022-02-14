use crate::{
    create_quiz, full_quiz, save_rounds, update_quiz, DraftQuiz, DraftRound, Error, FullDraftQuiz,
    User,
};
use std::rc::Rc;
use yew::Callback;

pub trait EmitError<Err> {
    type Ty;
    fn emit(self, callback: &Callback<Err>) -> Option<Self::Ty>
    where
        Self: Sized;
}

impl<T, E> EmitError<E> for Result<T, E> {
    type Ty = T;
    fn emit(self, callback: &Callback<E>) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(err) => {
                callback.emit(err);
                None
            }
        }
    }
}

pub struct ManagedQuiz {
    user: Option<User>,
    onerror: Callback<Error>,

    quiz: Option<FullDraftQuiz>,
    id: Option<u64>,
}

impl ManagedQuiz {
    pub async fn new(user: Option<User>, onerror: Callback<Error>, id: Option<u64>) -> Self {
        let cloned = user.clone();
        let quiz = match id {
            Some(id) => full_quiz(cloned, id).await.emit(&onerror).map(Into::into),
            None => None,
        };

        Self { user, onerror, quiz, id }
    }

    pub async fn change_quiz(&mut self, quiz: DraftQuiz) {
        let new = match &self.id {
            None => create_quiz(self.user.clone(), quiz).await,
            Some(id) => update_quiz(self.user.clone(), *id, quiz).await,
        }
        .emit(&self.onerror);

        if let (Some(quiz), Some(new)) = (&mut self.quiz, new) {
            quiz.quiz = new.into();
        }
    }

    pub async fn update_rounds(&mut self, mut rounds: Vec<DraftRound>) {
        for round in &mut rounds {
            round.image.as_mut().map(|x| x.upload());
        }

        if let Some(id) = self.id {
            // TODO: stuff
            save_rounds(self.user.clone(), id, rounds).await.emit(&self.onerror);
        }
    }
}
