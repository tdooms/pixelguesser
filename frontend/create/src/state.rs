use std::rc::Rc;

use yew::hook;
use yew::{use_state, UseStateHandle};

use api::{Quiz, Result, Round};
use shared::{spawn, use_auth, use_toast, UseToastHandle};

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
    async fn load(self, token: Option<String>, quiz_id: u32) {
        let result = Quiz::query_one(token, quiz_id, None).await;
        let mut quiz = match self.toast.api(result) {
            Some(quiz) => quiz,
            None => return,
        };

        quiz.rounds.resize(quiz.rounds.len().max(1), Default::default());
        let rc = Rc::new(quiz);

        self.prev.set(Rc::clone(&rc));
        self.quiz.set(rc);

        self.loading.set(false);
    }

    async fn upload(self, quiz: Rc<Quiz>, token: Rc<String>, creator_id: u64) {
        self.quiz.set(quiz.clone());
        self.prev.set(quiz.clone());

        let mut new = (*quiz).clone();
        new.creator_id = Some(creator_id);

        let _ = self.toast.api(new.image.upload((*token).clone()).await);

        for (index, round) in new.rounds.iter_mut().enumerate() {
            let _ = self.toast.api(round.image.upload((*token).clone()).await);
            round.round_index = index as u64
        }

        let rc = Rc::new(new);
        let result = match quiz.quiz_id {
            Some(_) => Quiz::update((*token).clone(), rc).await,
            None => Quiz::create((*token).clone(), rc).await,
        };

        if let Some(quiz) = self.toast.api(result) {
            self.quiz.set(Rc::new(quiz));
        }
    }

    async fn delete(self, token: Rc<String>) {
        let result = Quiz::delete((*token).clone(), (*self.quiz).clone()).await;
        let _ = self.toast.api(result);
    }

    pub fn quiz(&self) -> Rc<Quiz> {
        (*self.quiz).clone()
    }
    pub fn loading(&self) -> bool {
        *self.loading
    }
    pub fn changed(&self) -> bool {
        self.prev != self.quiz
    }

    pub fn action(&self, action: Action, token: Rc<String>, creator_id: u64) {
        let mut new = (**self.quiz).clone();
        let rounds = &mut new.rounds;
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
                spawn!(state.delete(token));
            }
            Action::Submit if self.changed() => {
                self.loading.set(true);
                let loading = self.loading.clone();

                spawn!(state; async move {
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

    let first = use_state(|| true);

    let prev = use_state(|| Rc::new(new.clone()));
    let quiz = use_state(|| Rc::new(new));

    let loading = use_state(|| quiz_id.is_some());
    let toast = use_toast();

    let res = UseQuizCreateHandle { prev, quiz, loading, toast };
    let token = use_auth().token().map(|x| (*x).clone());

    if let (true, Some(quiz_id)) = (*first, quiz_id) {
        first.set(false);
        spawn!(res; res.load(token, quiz_id))
    }

    res
}
