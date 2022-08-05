use api::{DraftQuiz, DraftRound, Result, User};
use std::rc::Rc;

use shared::{use_auth, use_startup, use_toast, UseToastHandle};
use yew::hook;
use yew::{use_state, UseStateHandle};

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
pub struct UseQuizCreateHandle {
    prev_quiz: UseStateHandle<Rc<DraftQuiz>>,
    quiz: UseStateHandle<Rc<DraftQuiz>>,

    quiz_id: UseStateHandle<Option<u32>>,
    loading: UseStateHandle<bool>,
    toast: UseToastHandle,
}

impl UseQuizCreateHandle {
    fn notify<T>(&self, result: Result<T>) -> Option<T> {
        result.map_err(|err| self.toast.add(err)).ok()
    }

    async fn load(self, quiz_id: u32, user: Option<Rc<User>>) {
        let quiz = match self.notify(api::query_quiz(user, quiz_id).await) {
            Some(quiz) => quiz,
            None => return,
        };

        let mut draft: DraftQuiz = quiz.into();
        draft.rounds.data.extend(draft.rounds.data.is_empty().then(|| DraftRound::default()));
        let draft = Rc::new(draft);

        self.prev_quiz.set(draft.clone());
        self.quiz.set(draft);

        self.loading.set(false);
    }

    async fn upload(self, draft: Rc<DraftQuiz>, user: Rc<User>) {
        self.quiz.set(draft.clone());
        self.prev_quiz.set(draft.clone());

        let mut inner = (*draft).clone();

        inner.creator_id = user.sub.clone();
        let _ = self.notify(inner.image.upload(user.sub.clone()).await);

        for (index, round) in inner.rounds.data.iter_mut().enumerate() {
            let _ = self.notify(round.image.upload(user.sub.clone()).await);
            round.index = index as u32
        }

        match *self.quiz_id {
            Some(quiz_id) => {
                let _ = self.notify(api::update_quiz(user, quiz_id, inner).await);
            }
            None => {
                let quiz = self.notify(api::create_quiz(user, inner).await);
                self.quiz_id.set(quiz.map(|x| x.id));
            }
        };
    }

    async fn delete(self, user: Rc<User>) {
        let _ = self.notify(api::delete_quiz(user, self.quiz_id.unwrap()).await);
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

    pub fn action(&self, action: Action, user: Rc<User>) {
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
                ywt::spawn!(state.delete(user));
            }
            Action::Submit if self.changed() => {
                self.loading.set(true);
                let loading = self.loading.clone();

                ywt::spawn!(state; async move {
                    let quiz = state.quiz();
                    state.upload(quiz, user).await;
                    loading.set(false);
                });
            }
            Action::Submit => {}
        }
    }
}

#[hook]
pub fn use_quiz_create(quiz_id: Option<u32>) -> UseQuizCreateHandle {
    let mut default = DraftQuiz::default();
    default.rounds.data.extend(default.rounds.data.is_empty().then(|| DraftRound::default()));

    let prev_quiz = use_state(|| Rc::new(default.clone()));
    let quiz = use_state(|| Rc::new(default));

    let loading = use_state(|| quiz_id.is_some());
    let quiz_id = use_state(|| quiz_id);

    let toast = use_toast();

    let res = UseQuizCreateHandle { prev_quiz, quiz, loading, quiz_id, toast };

    let user = use_auth().user();
    let cloned = res.clone();

    let startup = || {
        if let Some(quiz_id) = *cloned.quiz_id {
            ywt::spawn!(cloned.load(quiz_id, user))
        }
    };
    use_startup(startup);

    res
}
