use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew::suspense::{Suspension, SuspensionResult};

use api::{DraftQuiz, DraftRound, FullDraftQuiz, User};

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
    handle: UseStateHandle<Inner>,
    user: User,
}

impl UseCreateStateHandle {
    pub fn set_quiz(&self, quiz: DraftQuiz) {
        let mut inner = (*self.handle).clone();

        inner.full.quiz = quiz.clone();
        inner.stage = CreateStage::Rounds;

        let user = self.user.clone();
        spawn_local(async move {
            match inner.id {
                Some(id) => {
                    let _ = api::update_quiz(user, id, quiz).await;
                }
                None => {
                    let _ = api::create_quiz(user, quiz).await;
                }
            }
        });

        self.handle.set(inner)
    }

    pub fn set_stage(&self, stage: CreateStage) {
        let mut inner = (*self.handle).clone();
        inner.stage = stage;
        self.handle.set(inner)
    }

    pub fn set_rounds(&self, rounds: Vec<DraftRound>) {
        let mut inner = (*self.handle).clone();
        for round in &mut inner.full.rounds {
            round.image.as_mut().map(|x| x.upload());
        }

        inner.full.rounds = rounds.clone();

        let user = self.user.clone();
        let id = inner.id.unwrap();
        spawn_local(async move {
            let _ = api::save_rounds(user, id, rounds).await;
        });

        self.handle.set(inner)
    }

    pub fn delete(&self) {
        if let Some(id) = self.handle.id {
            let user = self.user.clone();
            spawn_local(async move {
                let _ = api::delete_quiz(user, id).await;
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
}

#[hook]
pub fn use_create_state(id: Option<u64>, user: User) -> SuspensionResult<UseCreateStateHandle> {
    log::info!("create state handle");
    let handle =
        use_state_eq(|| Inner { stage: CreateStage::Quiz, id, full: FullDraftQuiz::default() });
    let first = use_state_eq(|| true);
    let (suspense, cb) = Suspension::new();

    let (handle_clone, user_clone) = (handle.clone(), user.clone());
    if let Some(quiz_id) = id {
        let clone = first.clone();
        spawn_local(async move {
            let full = api::full_quiz(Some(user_clone), quiz_id).await.map(Into::into).unwrap();
            let inner = Inner { stage: CreateStage::Quiz, id, full };

            handle_clone.set(inner);
            clone.set(false);

            cb.resume();
        })
    } else {
        first.set(false)
    }

    match *first {
        false => Ok(UseCreateStateHandle { user, handle }),
        true => Err(suspense),
    }
}
