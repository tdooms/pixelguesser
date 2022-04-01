use cobul::use_value_state;
use cobul::Columns;
use shared::callback;
use yew::prelude::*;

use api::{DraftRound, Resolution};

use crate::round_edit::RoundEdit;

use crate::round_list::RoundList;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub rounds: Vec<DraftRound>,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onsave: Callback<Vec<DraftRound>>,
}

pub enum Action {
    Add,
    Delete,
    Select(usize),
    Edit(DraftRound),
}

pub fn change(
    state: UseStateHandle<Vec<DraftRound>>,
    current: UseStateHandle<usize>,
    action: Action,
    onsave: Callback<Vec<DraftRound>>,
) {
    let mut new = (*state).clone();
    let index = *current;

    match action {
        Action::Add => {
            current.set(new.len());
            new.push(DraftRound::default())
        }
        Action::Delete => {
            new.remove(index);
            current.set(1.max(index) - 1)
        }
        Action::Select(index) => current.set(index),
        Action::Edit(round) => new[index] = round,
    }
    onsave.emit(new)
}

pub fn maker<T: 'static>(
    local: &UseStateHandle<Vec<DraftRound>>,
    current: &UseStateHandle<usize>,
    onsave: &Callback<Vec<DraftRound>>,
    action: fn(T) -> Action,
) -> Callback<T> {
    callback!(local, current, onsave; move |x| change(local.clone(), current.clone(), action(x), onsave.clone()))
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { onback, ondone, onsave, .. } = props.clone();

    let rounds = match props.rounds.len() {
        0 => vec![DraftRound::default()],
        _ => props.rounds.clone(),
    };

    let local = use_value_state(&rounds);
    let current = use_state(|| 0_usize);

    let list = {
        let onselect = maker(&local, &current, &onsave, Action::Select);
        let onadd = maker(&local, &current, &onsave, |_| Action::Add);
        let ondelete = maker(&local, &current, &onsave, |_| Action::Delete);

        let images: Vec<_> = local
            .iter()
            .map(|round| round.image.as_ref().map(|img| img.src(Resolution::Thumbnail)))
            .collect();

        let current = *current;

        html! {<RoundList {onselect} {onadd} {ondelete} {images} {current}/>}
    };

    let edit = {
        let draft = local[*current].clone();
        let onedit = maker(&local, &current, &onsave, Action::Edit);

        html! {<RoundEdit {draft} {onback} {ondone} {onedit}/>}
    };

    html! {
        <Columns>
            {list}
            {edit}
        </Columns>
    }
}
