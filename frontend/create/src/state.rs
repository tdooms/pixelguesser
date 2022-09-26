use api::{Quiz, Result, Round};
use std::rc::Rc;

use shared::{use_auth, use_startup, use_toast, UseToastHandle};
use yew::hook;
use yew::{use_state, UseStateHandle};

pub enum Action {
    Quiz(Rc<Quiz>),
    Round(usize, Rc<Round>),
    Remove(usize),
    Add(usize),
    Swap(usize, usize),
    Submit,
    Delete,
}

#[derive(Clone)]
pub struct UseQuizCreateHandle {
    prev: UseStateHandle<Rc<Quiz>>,
    quiz: UseStateHandle<Rc<Quiz>>,

    loading: UseStateHandle<bool>,
    toast: UseToastHandle,
}

impl UseQuizCreateHandle {
    fn notify<T>(&self, result: Result<T>) -> Option<T> {
        result.map_err(|err| self.toast.add(err)).ok()
    }

    async fn load(self, quiz_id: u64, token: Option<String>) {
        let mut quiz = match self.notify(api::query_quiz(token, quiz_id).await) {
            Some(quiz) => quiz,
            None => return,
        };

        quiz.rounds.resize(quiz.rounds.len().min(1), Default::default());
        let quiz = Rc::new(draft);

        self.prev.set(quiz.clone());
        self.quiz.set(quiz);

        self.loading.set(false);
    }

    async fn upload(self, quiz: Rc<Quiz>, token: Rc<String>, creator_id: String) {
        self.quiz.set(quiz.clone());
        self.prev.set(quiz.clone());

        let mut new = (*quiz).clone();
        new.creator_id = Some(creator_id);
        let _ = self.notify(new.image.upload((*token).clone()).await);

        for (index, round) in new.rounds.data.iter_mut().enumerate() {
            let _ = self.notify(round.image.upload((*token).clone()).await);
            round.index = index as u32
        }

        match quiz.id {
            Some(_) => {
                let _ = self.notify(api::update_quiz((*token).clone(), new).await);
            }
            None => {
                let quiz = self.notify(api::create_quiz((*token).clone(), new).await);
                self.quiz_id.set(quiz.map(|x| x.id));
            }
        };
    }

    async fn delete(self, token: Rc<String>) {
        let _ = self.notify(api::delete_quiz((*token).clone(), self.quiz_id.unwrap()).await);
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

    pub fn action(&self, action: Action, token: Rc<String>, creator_id: String) {
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
                rounds.extend(rounds.is_empty().then(|| Default::default()));
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
                ywt::spawn!(state.delete(token));
            }
            Action::Submit if self.changed() => {
                self.loading.set(true);
                let loading = self.loading.clone();

                ywt::spawn!(state; async move {
                    let quiz = state.quiz();
                    state.upload(quiz, token, creator_id).await;
                    loading.set(false);
                });
            }
            Action::Submit => {}
        }
    }
}

#[hook]
pub fn use_quiz_create(quiz_id: Option<u32>) -> UseQuizCreateHandle {
    let mut new = Quiz::default();
    new.rounds.push(Round::default());

    let prev = use_state(|| Rc::new(new.clone()));
    let quiz = use_state(|| Rc::new(new));

    let loading = use_state(|| quiz_id.is_some());
    let toast = use_toast();

    let res = UseQuizCreateHandle { prev, quiz, loading, toast };

    let token = use_auth().token();
    let cloned = res.clone();

    let startup = || {
        if let Some(quiz_id) = *cloned.quiz_id {
            ywt::spawn!(cloned.load(quiz_id, token))
        }
    };
    use_startup(startup);

    res
}
