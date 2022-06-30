use api::{DraftQuiz, DraftRound, User};
use shared::{EmitError, Errors};

use wasm_bindgen_futures::spawn_local;
use yew::{hook, use_effect_with_deps};
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
    prev_rounds: UseStateHandle<Vec<DraftRound>>,
    rounds: UseStateHandle<Vec<DraftRound>>,

    prev_quiz: UseStateHandle<DraftQuiz>,
    quiz: UseStateHandle<DraftQuiz>,

    quiz_id: UseStateHandle<Option<u32>>,
    loading: UseStateHandle<bool>,
}

async fn load_quiz(
    state: CreateState,
    quiz_id: u32,
    user: Option<User>,
    err: Errors,
) -> Option<()> {
    let full = api::full_quiz(user, quiz_id).await.emit(&err)?;
    let mut rounds: Vec<_> = full.rounds.into_iter().map(DraftRound::from).collect();

    rounds.extend(rounds.is_empty().then(|| DraftRound::default()));

    state.prev_rounds.set(rounds.clone());
    state.rounds.set(rounds);

    state.prev_quiz.set(full.quiz.clone().into());
    state.quiz.set(full.quiz.into());

    state.loading.set(false);
    Some(())
}

async fn upload_quiz(state: CreateState, mut quiz: DraftQuiz, user: User, err: Errors) {
    // Upload the quiz image
    if let Some(image) = &mut quiz.image {
        let _ = image.upload().await.emit(&err);
    }
    quiz.creator_id = user.sub.clone();

    state.quiz.set(quiz.clone());
    state.prev_quiz.set(quiz.clone());

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

    // The quiz must exist before we can save rounds
    let quiz_id = state.quiz_id.unwrap();
    let _ = api::save_rounds(user, quiz_id, rounds.clone()).await.emit(&err);

    state.rounds.set(rounds.clone());
    state.prev_rounds.set(rounds);
}

#[hook]
pub fn use_create_state(
    callback: Callback<api::Error>,
    quiz_id: Option<u32>,
    user: Option<User>,
    err: Errors,
) -> CreateState {
    let prev_rounds = use_state(|| vec![]);
    let rounds = use_state(|| vec![DraftRound::default()]);

    let prev_quiz = use_state(|| DraftQuiz::default());
    let quiz = use_state(|| DraftQuiz::default());

    let loading = use_state(|| quiz_id.is_some());
    let quiz_id = use_state(|| quiz_id);

    let res = CreateState { prev_rounds, rounds, prev_quiz, quiz, loading, quiz_id };

    let cloned = res.clone();
    use_effect_with_deps(
        move |_| {
            if let Some(quiz_id) = *cloned.quiz_id {
                spawn_local(async move {
                    if let None = load_quiz(cloned, quiz_id, user, err).await {
                        callback.emit(api::Error::Empty);
                    }
                })
            }
            || ()
        },
        (),
    );

    res
}

impl CreateState {
    pub fn quiz(&self) -> DraftQuiz {
        (*self.quiz).clone()
    }

    pub fn rounds(&self) -> Vec<DraftRound> {
        (*self.rounds).clone()
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
            QuizAction::Delete => {
                spawn_local(async move { delete_quiz(cloned, user, err).await });
            }
            QuizAction::Submit if self.quiz != self.prev_quiz => {
                self.loading.set(true);
                let (quiz, loading) = (self.quiz(), self.loading.clone());
                spawn_local(async move {
                    upload_quiz(cloned, quiz, user, err).await;
                    loading.set(false);
                });
            }
            QuizAction::Submit => {}
        }
    }

    pub fn set_rounds(&self, action: RoundsAction, user: User, err: Errors) {
        let mut rounds = (*self.rounds).clone();

        match action {
            RoundsAction::Edit(index, round) => {
                rounds[index] = round;
            }
            RoundsAction::Remove(index) => {
                rounds.remove(index);
                rounds.extend(rounds.is_empty().then(|| DraftRound::default()));
            }
            RoundsAction::Add(index) => {
                rounds.insert(index + 1, Default::default());
            }
            RoundsAction::Swap(from, to) => {
                let round = rounds.remove(from);
                rounds.insert(to, round);
            }
            RoundsAction::Submit if self.rounds != self.prev_rounds => {
                let (cloned, rounds) = (self.clone(), rounds.clone());
                spawn_local(async move { upload_rounds(cloned, rounds, user, err).await });
            }
            RoundsAction::Submit => {}
        };

        self.rounds.set(rounds)
    }
}
