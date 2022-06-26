use api::{DraftQuiz, DraftRound, FullDraftQuiz, User};
use shared::{EmitError, Error, Errors};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Mutex;
use wasm_bindgen_futures::spawn_local;
use yew::hook;
use yew::{use_state, Callback, UseStateHandle};

pub enum RoundsAction {
    Edit(usize, DraftRound),
    Remove(usize),
    Add(usize),
    Swap(usize, usize),
    Submit,
}

pub enum QuizAction {
    Edit(DraftQuiz),
    Submit,
    Delete,
}

#[derive(Clone)]
pub struct CreateState {
    previous: UseStateHandle<Vec<DraftRound>>,
    current: UseStateHandle<Vec<DraftRound>>,
    quiz: UseStateHandle<DraftQuiz>,

    quiz_id: UseStateHandle<Option<u32>>,
    loading: UseStateHandle<bool>,
}

async fn load_quiz(state: CreateState, quiz_id: u32, user: Option<User>, err: Errors) {
    let full = api::full_quiz(user, quiz_id).await.emit(&err).unwrap();
    let rounds: Vec<_> = full.rounds.into_iter().map(DraftRound::from).collect();
    state.previous.set(rounds.clone());
    state.current.set(rounds);
    state.quiz.set(full.quiz.into());
    state.loading.set(false);
}

async fn upload_quiz(state: CreateState, quiz: DraftQuiz, user: User, err: Errors) {
    state.quiz.set(quiz.clone());

    match *state.quiz_id {
        Some(id) => api::update_quiz(user, id, quiz).await.emit(&err),
        None => api::create_quiz(user, quiz).await.emit(&err),
    };
}

async fn delete_quiz(state: CreateState, user: User, err: Errors) {
    api::delete_quiz(user, state.quiz_id.unwrap()).await.emit(&err);
}

async fn upload_rounds(state: CreateState, mut rounds: Vec<DraftRound>, user: User, err: Errors) {
    for image in rounds.iter_mut().filter_map(|round| round.image.as_mut()) {
        let _ = image.upload().await.emit(&err);
    }

    let quiz_id = state.quiz_id.unwrap();
    let _ = api::save_rounds(user, quiz_id, rounds.clone()).await.emit(&err);

    state.current.set(rounds.clone());
    state.previous.set(rounds);
}

#[hook]
pub fn use_create_state(
    callback: Callback<api::Error>,
    quiz_id: Option<u32>,
    user: Option<User>,
    err: Errors,
) -> CreateState {
    let previous = use_state(|| vec![]);
    let current = use_state(|| vec![DraftRound::default()]);
    let quiz = use_state(|| DraftQuiz::default());
    let loading = use_state(|| quiz_id.is_some());
    let quiz_id = use_state(|| quiz_id);

    let res = CreateState { previous, current, quiz, loading, quiz_id: quiz_id.clone() };
    if let Some(quiz_id) = *quiz_id {
        let cloned = res.clone();
        spawn_local(async move { load_quiz(cloned, quiz_id, user, err).await })
    }

    res
}

impl CreateState {
    pub fn quiz(&self) -> DraftQuiz {
        (*self.quiz).clone()
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        (*self.current).clone()
    }
    pub fn loading(&self) -> bool {
        *self.loading
    }

    pub fn set_quiz(&self, action: QuizAction, user: User, err: Errors) {
        let cloned = self.clone();
        match action {
            QuizAction::Edit(quiz) => {
                self.quiz.set(quiz);
            }
            QuizAction::Submit => {
                let quiz = self.quiz();
                spawn_local(async move { upload_quiz(cloned, quiz, user, err).await });
            }
            QuizAction::Delete => spawn_local(async move { delete_quiz(cloned, user, err).await }),
        }
    }

    pub fn set_rounds(&self, action: RoundsAction, user: User, err: Errors) {
        let mut rounds = (*self.current).clone();

        match action {
            RoundsAction::Edit(index, round) => {
                rounds[index] = round;
            }
            RoundsAction::Remove(index) => {
                rounds.remove(index);
            }
            RoundsAction::Add(index) => {
                rounds.insert(index, Default::default());
            }
            RoundsAction::Swap(from, to) => {
                let round = rounds.remove(from);
                rounds.insert(to, round);
            }
            RoundsAction::Submit => {
                let (cloned, rounds) = (self.clone(), rounds.clone());
                spawn_local(async move { upload_rounds(cloned, rounds, user, err).await });
            }
        };

        self.current.set(rounds)
    }
}
