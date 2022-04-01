use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use api::{DraftQuiz, DraftRound, FullDraftQuiz, User};
use shared::{EmitError, Error, Errors};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CreateStage {
    Quiz,
    Rounds,
    Summary,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Inner {
    id: Option<u64>,
    full: FullDraftQuiz,
    stage: CreateStage,
}

#[derive(Clone)]
pub struct UseCreateStateHandle {
    handle: UseStateHandle<Inner>,
    user: User,
    errors: Errors,
}

impl UseCreateStateHandle {
    pub fn set_quiz(&self, quiz: DraftQuiz) {
        let Self { handle, user, errors } = self.clone();
        let mut inner = (*handle).clone();

        inner.full.quiz = quiz.clone();
        inner.stage = CreateStage::Rounds;

        spawn_local(async move {
            let result = match inner.id {
                Some(id) => api::update_quiz(user, id, quiz).await,
                None => api::create_quiz(user, quiz).await,
            };

            // TODO: use quiz for something else?
            let quiz = result.map_err(Error::Api).emit(&errors);

            log::warn!("{:?}", inner);
            inner.id = quiz.flatten().map(|quiz| quiz.id);
            handle.set(inner);
        });
    }

    pub fn set_stage(&self, stage: CreateStage) {
        let mut inner = (*self.handle).clone();
        inner.stage = stage;
        log::warn!("{:?}", inner);
        self.handle.set(inner)
    }

    pub fn set_rounds(&self, rounds: Vec<DraftRound>) {
        let Self { user, errors, .. } = self.clone();
        let mut inner = (*self.handle).clone();
        let id = inner.id.unwrap();

        inner.full.rounds = rounds.clone();

        for round in &mut inner.full.rounds {
            round.image.as_mut().map(|x| x.upload());
        }

        log::warn!("settings set_rounds");
        self.handle.set(inner);

        spawn_local(async move {
            let result = api::save_rounds(user, id, rounds).await;
            let _ = result.map_err(Error::Api).emit(&errors);
        });
    }

    pub fn delete(&self, callback: impl FnOnce() + 'static) {
        if let Some(id) = self.handle.id {
            let Self { user, errors, .. } = self.clone();

            spawn_local(async move {
                let result = api::delete_quiz(user, id).await;
                let _ = result.map_err(Error::Api).emit(&errors).unwrap();
                callback()
            })
        }
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        self.handle.full.rounds.clone()
    }

    pub fn quiz(&self) -> DraftQuiz {
        self.handle.full.quiz.clone()
    }

    pub fn stage(&self) -> CreateStage {
        self.handle.stage.clone()
    }

    pub fn id(&self) -> Option<u64> {
        self.handle.id.clone()
    }
}

#[hook]
pub fn use_create_state(
    id: Option<u64>,
    user: User,
    errors: Errors,
) -> SuspensionResult<UseCreateStateHandle> {
    let handle = use_state_eq(|| {
        log::warn!("changing stage");
        Inner { stage: CreateStage::Quiz, id, full: FullDraftQuiz::default() }
    });
    let first = use_state_eq(|| true);
    let (suspense, cb) = Suspension::new();

    let (handle_clone, user_clone) = (handle.clone(), user.clone());
    if let Some(quiz_id) = id {
        let clone_first = first.clone();
        spawn_local(async move {
            let full = api::full_quiz(Some(user_clone), quiz_id).await.map(Into::into).unwrap();
            let inner = Inner { stage: (*handle_clone).stage, id, full };

            log::warn!("{:?}", inner);
            handle_clone.set(inner);
            clone_first.set(false);

            cb.resume();
        })
    } else {
        first.set(false)
    }

    match *first {
        false => Ok(UseCreateStateHandle { user, handle, errors }),
        true => Err(suspense),
    }
}
