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

impl Default for CreateStage {
    fn default() -> Self {
        CreateStage::Quiz
    }
}

#[derive(Clone, PartialEq)]
pub struct UseCreateStateHandle {
    id: UseStateHandle<Option<u64>>,
    full: UseStateHandle<FullDraftQuiz>,
    old: UseStateHandle<FullDraftQuiz>,
    stage: UseStateHandle<CreateStage>,

    user: User,
    errors: Errors,
}

impl UseCreateStateHandle {
    pub fn set_quiz(&self, mut quiz: DraftQuiz) {
        let mut inner = (*self.full).clone();
        self.full.set(FullDraftQuiz { quiz, ..inner });
    }

    pub fn submit_quiz(&self) {
        self.stage.set(CreateStage::Rounds);
        if &self.old.quiz == &self.full.quiz {
            return;
        }

        let Self { id, full, old, user, errors, .. } = self.clone();

        spawn_local(async move {
            let mut quiz = full.quiz.clone();

            if let Some(image) = &mut quiz.image {
                let _ = image.upload().await.map_err(Error::Api).emit(&errors);
            }

            let result = match *id {
                Some(id) => api::update_quiz(user, id, quiz.clone()).await,
                None => api::create_quiz(user, quiz.clone()).await,
            };

            let result = result.map_err(Error::Api).emit(&errors);

            id.set(id.or(result.flatten().map(|quiz| quiz.id)));

            full.set(FullDraftQuiz { quiz: quiz.clone(), ..(*full).clone() });
            old.set(FullDraftQuiz { quiz, ..(*full).clone() });
        });
    }

    pub fn set_stage(&self, stage: CreateStage) {
        self.stage.set(stage)
    }

    pub fn set_rounds(&self, mut rounds: Vec<DraftRound>) {
        let mut inner = (*self.full).clone();
        self.full.set(FullDraftQuiz { rounds, ..inner });
    }

    pub fn submit_rounds(&self) {
        if &self.old.rounds == &self.full.rounds {
            return;
        }

        let Self { user, errors, full, old, id, .. } = self.clone();
        spawn_local(async move {
            let mut rounds = full.rounds.clone();

            for image in rounds.iter_mut().filter_map(|round| round.image.as_mut()) {
                let _ = image.upload().await.map_err(Error::Api).emit(&errors);
            }

            let result = api::save_rounds(user, (*id).unwrap(), rounds.clone()).await;
            let _ = result.map_err(Error::Api).emit(&errors);

            full.set(FullDraftQuiz { rounds: rounds.clone(), ..(*full).clone() });
            old.set(FullDraftQuiz { rounds, ..(*full).clone() });
        });
    }

    pub fn delete(&self, callback: impl FnOnce() + 'static) {
        if let Some(id) = *self.id {
            let Self { user, errors, .. } = self.clone();

            spawn_local(async move {
                let result = api::delete_quiz(user, id).await;
                let _ = result.map_err(Error::Api).emit(&errors).unwrap();
                callback()
            })
        }
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        self.full.rounds.clone()
    }

    pub fn quiz(&self) -> DraftQuiz {
        self.full.quiz.clone()
    }

    pub fn stage(&self) -> CreateStage {
        (*self.stage).clone()
    }

    pub fn id(&self) -> Option<u64> {
        (*self.id).clone()
    }
}

#[hook]
pub fn use_create_state(
    quiz_id: Option<u64>,
    user: User,
    errors: Errors,
) -> SuspensionResult<UseCreateStateHandle> {
    let full = use_state_eq(FullDraftQuiz::default);
    let old = use_state_eq(FullDraftQuiz::default);
    let stage = use_state_eq(CreateStage::default);
    let id = use_state_eq(|| quiz_id);

    let first = use_state_eq(|| true);

    match (*first, quiz_id) {
        (true, Some(quiz_id)) => Err(Suspension::from_future(async move {
            let response: FullDraftQuiz =
                api::full_quiz(Some(user), quiz_id).await.map(Into::into).unwrap();

            full.set(response.clone());
            old.set(response);

            first.set(false);
        })),
        _ => Ok(UseCreateStateHandle { full, old, stage, id, user, errors }),
    }
}
