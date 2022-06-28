use cobul::Columns;
use futures::stream::iter;
use std::collections::HashMap;
use yew::prelude::*;

use api::{DraftRound, Resolution};
use shared::callback;

use crate::round_edit::RoundEdit;
use crate::round_list::RoundList;
use crate::state::RoundsAction;
use crate::Stage;

pub fn check_rounds(rounds: &[DraftRound]) -> Vec<bool> {
    let incomplete = |round: &DraftRound| round.answer.is_empty() || round.image.is_none();
    rounds.iter().map(incomplete).collect()
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onchange: Callback<RoundsAction>,
    pub rounds: Vec<DraftRound>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { onstage, onchange, rounds } = props.clone();
    let current = use_state(|| 0usize);
    let draft = rounds[*current].clone();
    let incomplete = check_rounds(&rounds);

    let ondone = callback!(onstage, onchange, incomplete; move |_| {
        if incomplete.iter().all(|x| !x) {
            onstage.emit(Stage::Summary);
            onchange.emit(RoundsAction::Submit);
        }
    });

    let onback = callback!(onstage; move |_| {
        onstage.emit(Stage::Quiz)
    });

    let list = {
        let onselect = callback!(current; move |idx| {
            current.set(idx);
        });
        let onadd = callback!(current, onchange; move |_| {
            onchange.emit(RoundsAction::Add(*current));
            current.set(*current + 1);
        });
        let onremove = callback!(current, onchange; move |idx| {
            onchange.emit(RoundsAction::Remove(idx));
            current.set(*current - current.min((idx <= *current) as usize));
        });

        let images: Vec<_> = rounds
            .iter()
            .map(|round| round.image.as_ref().map(|x| x.src(Resolution::Thumbnail)))
            .collect();

        let (current, incomplete) = (*current, incomplete.clone());
        html! {<RoundList {onselect} {onadd} {onremove} {images} {current} {incomplete}/>}
    };

    let edit = {
        let onedit = callback!(current; move |round| {
            onchange.emit(RoundsAction::Edit(*current, round));
        });

        html! {<RoundEdit {draft} {onback} {ondone} {onedit}/>}
    };

    html! {
        <Columns>
            {list}
            {edit}
        </Columns>
    }
}
