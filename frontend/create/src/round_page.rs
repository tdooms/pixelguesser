use cobul::use_value_state;
use cobul::Columns;
use futures::FutureExt;
use shared::callback;
use yew::prelude::*;

use api::{DraftRound, Image, Resolution};
use shared::{set_timer, Error};

use crate::round_edit::RoundEdit;
use crate::round_form::{RoundForm, RoundInfo};
use crate::round_list::RoundList;
use crate::round_preview::RoundPreview;

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
    Remove,
    Select(usize),
    Upload(Image),
    Edit(DraftRound),
}

pub fn change(
    state: UseStateHandle<Vec<DraftRound>>,
    current: UseStateHandle<usize>,
    action: Action,
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
        Action::Upload(image) => new[index].image = Some(image),
        Action::Edit(round) => new[index] = round,
        Action::Remove => new[index].image = None,
    }
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { onback, ondone, onsave, .. } = props.clone();
    let local = use_value_state(&props.rounds);
    let current = use_state(|| 0_usize);

    let list = {
        let onselect = callback!(local, current; move |idx| change(local.clone(), current.clone(), Action::Select(idx)));
        let onadd =
            callback!(local, current; move |_| change(local.clone(), current.clone(), Action::Add));
        let ondelete = callback!(local, current; move |_| change(local.clone(), current.clone(), Action::Delete));

        let images: Vec<_> = local
            .iter()
            .map(|round| round.image.as_ref().map(|img| img.src(Resolution::Thumbnail)))
            .collect();

        let current = *current;

        html! {<RoundList {onselect} {onadd} {ondelete} {images} {current}/>}
    };

    let edit = {
        let draft = local[*current].clone();
        let onedit = callback!(local, current; move |round| change(local.clone(), current.clone(), Action::Edit(round)));

        html! {<RoundEdit {draft} {onback} {ondone} {onedit}/>}
    };

    html! {
        <Columns>
            {list}
            {edit}
        </Columns>
    }
}
