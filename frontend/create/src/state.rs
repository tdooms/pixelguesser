use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use api::{DraftQuiz, DraftRound, FullDraftQuiz, Stage, User};
use shared::{use_async_suspension, EmitError, Error};

#[derive(Clone, Copy, PartialEq)]
pub enum CreateStage {
    Quiz,
    Rounds,
    Summary,
}

#[derive(Clone, PartialEq)]
pub struct Inner {
    id: Option<u64>,
    full: FullDraftQuiz,
    stage: CreateStage,
}

#[derive(Clone)]
pub struct UseCreateStateHandle {
    handle: UseStateHandle<Option<Inner>>,
    user: User,
}

impl UseCreateStateHandle {
    pub fn set_quiz(&self, quiz: DraftQuiz) {
        if let Some(mut inner) = (*self.handle).clone() {
            inner.full.quiz = quiz;
            self.handle.set(Some(inner))
        }
    }

    pub fn set_stage(&self, stage: CreateStage) {
        if let Some(mut inner) = (*self.handle).clone() {
            inner.stage = stage;
            self.handle.set(Some(inner))
        }
    }

    pub fn set_rounds(&self, mut rounds: Vec<DraftRound>) {
        if let Some(mut inner) = (*self.handle).clone() {
            for round in &mut rounds {
                round.image.as_mut().map(|x| x.upload());
            }

            inner.full.rounds = rounds;
            self.handle.set(Some(inner))
        }
    }

    pub fn delete(&self) {
        let stored = self.handle.as_ref().map(|s| s.id).clone().flatten();
        if let Some(id) = stored {
            let user = self.user.clone();
            spawn_local(async move {
                api::delete_quiz(user, id).await;
            })
        }
        self.handle.set(None)
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        self.handle.as_ref().map(|s| s.full.rounds.clone()).unwrap()
    }

    pub fn quiz(&self) -> DraftQuiz {
        self.handle.as_ref().map(|s| s.full.quiz.clone()).unwrap()
    }

    pub fn stage(&self) -> CreateStage {
        self.handle.as_ref().map(|s| s.stage.clone()).unwrap()
    }
}

#[hook]
pub fn use_create_state(id: Option<u64>, user: User) -> SuspensionResult<UseCreateStateHandle> {
    let handle = use_state(|| None);
    let (suspense, cb) = Suspension::new();

    let (handle_clone, user_clone) = (handle.clone(), user.clone());
    match id {
        Some(quiz_id) => spawn_local(async move {
            let full = api::full_quiz(Some(user_clone), quiz_id).await.map(Into::into).unwrap();
            let inner = Inner { stage: CreateStage::Quiz, id, full };
            handle_clone.set(Some(inner));
            cb.resume();
        }),
        None => {
            let inner = Inner { stage: CreateStage::Quiz, id, full: FullDraftQuiz::default() };
            handle_clone.set(Some(inner));
        }
    }

    match handle.as_ref() {
        Some(_) => Ok(UseCreateStateHandle { user, handle }),
        None => Err(suspense),
    }
}
