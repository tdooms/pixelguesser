use std::rc::Rc;

use cobul::Columns;
use yew::prelude::*;

use api::{DraftRound, Resolution};
use shared::callback;

use crate::round_edit::RoundEdit;
use crate::round_list::RoundList;
use crate::state::{CreateStage, UseCreateStateHandle};

pub enum Action {
    Add,
    Delete,
    Select(usize),
    Edit(DraftRound),
}

#[derive(Clone, PartialEq)]
struct State {
    pub rounds: Rc<Vec<DraftRound>>,
    pub changes: UseStateHandle<u64>,
    pub current: UseStateHandle<usize>,
}

fn update(state: State, action: Action, onchange: Callback<Vec<DraftRound>>, onsave: Callback<()>) {
    let mut new = (*state.rounds).clone();
    let index = *state.current;

    match action {
        Action::Add => {
            state.current.set(new.len());
            new.push(DraftRound::default())
        }
        Action::Delete => {
            new.remove(index);
            state.current.set(1.max(index) - 1)
        }
        Action::Select(index) => {
            if *state.current != index {
                onsave.emit(())
            }
            state.current.set(index);
        }
        Action::Edit(round) => {
            new[index] = round;
            state.changes.set(0)
        }
    }
    onchange.emit(new);
    state.changes.set(*state.changes + 1);
}

fn maker<T: 'static>(
    state: &State,
    onsave: &Callback<()>,
    onchange: &Callback<Vec<DraftRound>>,
    action: fn(T) -> Action,
) -> Callback<T> {
    callback!(state, onsave, onchange; move |x| update(state.clone(), action(x), onchange.clone(), onsave.clone()))
}

#[function_component(RoundPage)]
pub fn round_page() -> Html {
    log::trace!("round page render");
    let create_state = use_context::<UseCreateStateHandle>().unwrap();

    let onback = callback!(create_state; move |_| create_state.set_stage(CreateStage::Quiz));
    let ondone = callback!(create_state; move |_| {
        create_state.set_stage(CreateStage::Summary);
        create_state.submit_rounds();
    });
    let onsave = callback!(create_state; move |_| create_state.submit_rounds());
    let onchange = callback!(create_state; move |rounds| create_state.set_rounds(rounds));

    let rounds = create_state.rounds();
    let rounds = match rounds.len() {
        0 => Rc::new(vec![DraftRound::default()]),
        _ => Rc::new(rounds),
    };

    let current = use_state(|| 0_usize);
    let changes = use_state(|| 0_u64);
    let state = State { rounds, current, changes };

    let list = {
        let onselect = maker(&state, &onsave, &onchange, Action::Select);
        let onadd = maker(&state, &onsave, &onchange, |_| Action::Add);
        let ondelete = maker(&state, &onsave, &onchange, |_| Action::Delete);

        let images: Vec<_> = state
            .rounds
            .iter()
            .map(|round| round.image.as_ref().map(|x| x.src(Resolution::Thumbnail)))
            .collect();

        let current = *state.current;
        html! {<RoundList {onselect} {onadd} {ondelete} {images} {current}/>}
    };

    let edit = {
        let draft = state.rounds[*state.current].clone();
        let onedit = maker(&state, &onsave, &onchange, Action::Edit);

        html! {<RoundEdit {draft} {onback} {ondone} {onedit}/>}
    };

    html! {
        <Columns>
            {list}
            {edit}
        </Columns>
    }
}
