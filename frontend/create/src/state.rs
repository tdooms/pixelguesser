use api::{DraftQuiz, DraftRound, User};
use shared::{EmitError, Errors};
use std::rc::Rc;

use wasm_bindgen_futures::spawn_local;
use yew::{hook, use_effect_with_deps};
use yew::{use_state, Callback, UseStateHandle};
use ywt::spawn;

pub enum Action {
    Quiz(Rc<DraftQuiz>),
    Round(usize, Rc<DraftRound>),
    Remove(usize),
    Add(usize),
    Swap(usize, usize),
    Submit,
    Delete,
}

#[derive(Clone)]
pub struct CreateState {
    prev_quiz: UseStateHandle<Rc<DraftQuiz>>,
    quiz: UseStateHandle<Rc<DraftQuiz>>,

    quiz_id: UseStateHandle<Option<u32>>,
    loading: UseStateHandle<bool>,
}

impl CreateState {
    async fn load(self, quiz_id: u32, user: Option<User>, err: Errors) -> Option<()> {
        let quiz = api::query_quiz(user, quiz_id).await.emit(&err)?;

        let mut draft: DraftQuiz = quiz.into();
        draft.rounds.data.extend(draft.rounds.data.is_empty().then(|| DraftRound::default()));
        let draft = Rc::new(draft);

        self.prev_quiz.set(draft.clone());
        self.quiz.set(draft);

        self.loading.set(false);
        Some(())
    }

    async fn upload(self, draft: Rc<DraftQuiz>, user: User, err: Errors) {
        self.quiz.set(draft.clone());
        self.prev_quiz.set(draft.clone());

        let mut inner = (*draft).clone();

        inner.creator_id = user.sub.clone();
        inner.image.upload(user.sub.clone()).await.emit(&err);

        for (index, round) in inner.rounds.data.iter_mut().enumerate() {
            round.image.upload(user.sub.clone()).await.emit(&err);
            round.index = index as u32
        }

        match *self.quiz_id {
            Some(quiz_id) => {
                api::update_quiz(user, quiz_id, inner).await.emit(&err);
            }
            None => {
                let quiz = api::create_quiz(user, inner).await.emit(&err);
                self.quiz_id.set(quiz.map(|x| x.id));
            }
        };
    }

    async fn delete(self, user: User, err: Errors) {
        api::delete_quiz(user, self.quiz_id.unwrap()).await.emit(&err);
    }

    pub fn quiz(&self) -> Rc<DraftQuiz> {
        (*self.quiz).clone()
    }
    pub fn loading(&self) -> bool {
        *self.loading
    }
    pub fn changed(&self) -> bool {
        self.prev_quiz != self.quiz
    }

    pub fn action(&self, action: Action, user: User, err: Errors) {
        let mut new = (**self.quiz).clone();
        let rounds = &mut new.rounds.data;
        let state = self.clone();

        match action {
            Action::Quiz(quiz) => {
                self.quiz.set(quiz);
            }
            Action::Round(index, round) => {
                rounds[index] = (*round).clone();
                self.quiz.set(Rc::new(new));
            }
            Action::Remove(index) => {
                rounds.remove(index);
                rounds.extend(rounds.is_empty().then(|| DraftRound::default()));
                self.quiz.set(Rc::new(new));
            }
            Action::Add(index) => {
                rounds.insert(index + 1, Default::default());
                self.quiz.set(Rc::new(new));
            }
            Action::Swap(from, to) => {
                let round = rounds.remove(from);
                rounds.insert(to, round);
                self.quiz.set(Rc::new(new));
            }
            Action::Delete => {
                spawn!(state.delete(user, err));
            }
            Action::Submit if self.changed() => {
                self.loading.set(true);
                let loading = self.loading.clone();

                spawn!(state; async move {
                    let quiz = state.quiz();
                    state.upload(quiz, user, err).await;
                    loading.set(false);
                });
            }
            Action::Submit => {}
        }
    }
}

#[hook]
pub fn use_create_state(
    callback: Callback<api::Error>,
    quiz_id: Option<u32>,
    user: Option<User>,
    err: Errors,
) -> CreateState {
    let mut default = DraftQuiz::default();
    default.rounds.data.extend(default.rounds.data.is_empty().then(|| DraftRound::default()));

    let prev_quiz = use_state(|| Rc::new(default.clone()));
    let quiz = use_state(|| Rc::new(default));

    let loading = use_state(|| quiz_id.is_some());
    let quiz_id = use_state(|| quiz_id);

    let res = CreateState { prev_quiz, quiz, loading, quiz_id };

    let cloned = res.clone();
    use_effect_with_deps(
        move |_| {
            if let Some(quiz_id) = *cloned.quiz_id {
                spawn_local(async move {
                    if let None = cloned.load(quiz_id, user, err).await {
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
