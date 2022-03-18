use std::rc::Rc;

use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Columns, Icon, Icons, Sidebar};
use futures::FutureExt;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

use api::{DraftQuiz, DraftRound, Image, Resolution};
use shared::{reduce_callback, set_timer, Error, CREATE_LONG_SAVE_TIME, CREATE_SHORT_SAVE_TIME};

use crate::round_edit::RoundEdit;
use crate::round_form::{RoundForm, RoundInfo};
use crate::round_list::RoundList;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub rounds: Vec<DraftRound>,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onsave: Callback<Vec<DraftRound>>,
}

#[derive(Debug, Clone)]
pub struct State {
    rounds: Vec<DraftRound>,
    current: usize,
}

impl Default for State {
    fn default() -> Self {
        Self { rounds: vec![DraftRound::default()], current: 0 }
    }
}

pub enum Action {
    Delete,
    Add,
    Select(usize),
    Move,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let new = Rc::make_mut(&mut self);
        match action {
            Action::Delete => {
                new.rounds.remove(new.current);
                new.current = new.current.min(new.rounds.len() - 1);
            }
            Action::Add => {
                new.current = new.rounds.len();
                new.rounds.push(DraftRound::default());
            }
            Action::Select(index) => new.current = index,
            Action::Move => {}
        }
        Rc::clone(&self)
    }
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { rounds, onback, ondone, onsave } = props.clone();

    let state = use_reducer(State::default);

    let onedit = {
        let rounds = state.rounds.clone();
        Callback::from(move |_| onsave.emit(rounds.clone()))
    };

    let left = {
        let images: Vec<_> = state
            .rounds
            .iter()
            .map(|x| x.image.as_ref().map(|x| x.src(Resolution::Thumbnail)))
            .collect();

        let onadd = reduce_callback(&state, |_| Action::Add);
        let ondelete = reduce_callback(&state, |_| Action::Delete);
        let onselect = reduce_callback(&state, Action::Select);

        html! {
            <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} class="p-0" overflow=true>
                <RoundList {images} {onselect} {ondelete} {onadd} current={state.current}/>
            </Sidebar>
        }
    };

    let edit = {
        let draft = state.rounds[state.current].clone();
        html! {<RoundEdit {onback} {ondone} {draft} {onedit} />}
    };

    html! {
        <Columns>
            {left}
            {edit}
        </Columns>
    }
}
