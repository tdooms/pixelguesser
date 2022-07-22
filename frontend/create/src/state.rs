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

impl CreateState {
    async fn load_quiz(self, quiz_id: u32, user: Option<User>, err: Errors) -> Option<()> {
        let full = api::full_quiz(user, quiz_id).await.emit(&err)?;

        let mut rounds: Vec<_> = full.rounds.into_iter().map(DraftRound::from).collect();
        rounds.extend(rounds.is_empty().then(|| DraftRound::default()));

        self.prev_rounds.set(rounds.clone());
        self.rounds.set(rounds);

        self.prev_quiz.set(full.quiz.clone().into());
        self.quiz.set(full.quiz.into());

        self.loading.set(false);
        Some(())
    }

    async fn upload_quiz(self, mut draft: DraftQuiz, user: User, err: Errors) {
        draft.creator_id = user.sub.clone();
        draft.image.upload().await.emit(&err);

        self.quiz.set(draft.clone());
        self.prev_quiz.set(draft.clone());

        match *self.quiz_id {
            Some(quiz_id) => {
                api::update_quiz(user, quiz_id, draft).await.emit(&err);
            }
            None => {
                let quiz = api::create_quiz(user, draft).await.emit(&err).flatten();
                self.quiz_id.set(quiz.map(|x| x.id));
            }
        };
    }

    async fn delete_quiz(self, user: User, err: Errors) {
        api::delete_quiz(user, self.quiz_id.unwrap()).await.emit(&err);
    }

    async fn upload_rounds(self, mut rounds: Vec<DraftRound>, user: User, err: Errors) {
        for round in &mut rounds {
            round.image.upload().await.emit(&err);
        }

        // The quiz must exist before we can save rounds
        let quiz_id = self.quiz_id.unwrap();
        let _ = api::save_rounds(user, quiz_id, rounds.clone()).await.emit(&err);

        self.rounds.set(rounds.clone());
        self.prev_rounds.set(rounds);
    }
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
                    if let None = cloned.load_quiz(quiz_id, user, err).await {
                        callback.emit(api::Error::EmptyResponse);
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
                spawn_local(async move { cloned.delete_quiz(user, err).await });
            }
            QuizAction::Submit if self.quiz != self.prev_quiz => {
                self.loading.set(true);
                let (quiz, loading) = (self.quiz(), self.loading.clone());

                spawn_local(async move {
                    cloned.upload_quiz(quiz, user, err).await;
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
                spawn_local(async move { cloned.upload_rounds(rounds, user, err).await });
            }
            RoundsAction::Submit => {}
        };

        self.rounds.set(rounds)
    }
}
