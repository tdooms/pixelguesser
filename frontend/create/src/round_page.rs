use std::rc::Rc;

use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::use_value_state;
use cobul::{Button, Buttons, Column, Columns, Icon, Icons, Sidebar};
use futures::FutureExt;
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use api::{DraftRound, Image, Resolution};
use shared::{reduce_callback, set_timer, Error, CREATE_LONG_SAVE_TIME, CREATE_SHORT_SAVE_TIME};

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

enum Action {
    Add,
    Delete,
    Remove,
    Select(usize),
    Upload(Image),
    Edit(DraftRound),
}

pub fn change(
    (state, current): (UseStateHandle<Vec<DraftRound>>, UseStateHandle<usize>),
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

    let cloner = || (local.clone(), current.clone());

    let list = {
        let onselect = Callback::from(|idx| change(cloner(), Action::Select(idx)));
        let onadd = Callback::from(|_| change(cloner(), Action::Add));
        let ondelete = Callback::from(|_| change(cloner(), Action::Delete));

        let images: Vec<_> = local
            .iter()
            .map(|round| round.image.map(|img| img.src(Resolution::Thumbnail)))
            .collect();

        let current = *current;

        html! {<RoundList {onselect} {onadd} {ondelete} {images} {current}/>}
    };

    let edit = {
        let draft = local[*current].clone();
        let onedit = Callback::from(|round| change(cloner(), Action::Edit(round)));

        html! {<RoundEdit {draft} {onback} {ondone} {onedit}/>}
    };

    html! {
        <Columns>
            {list}
            {edit}
        </Columns>
    }
}
