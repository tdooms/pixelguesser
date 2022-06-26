use std::rc::Rc;

use cobul::Columns;
use yew::prelude::*;

use api::{DraftRound, Resolution};
use shared::callback;

use crate::round_edit::RoundEdit;
use crate::round_list::RoundList;
use crate::state::RoundsAction;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onchange: Callback<RoundsAction>,
    pub rounds: Vec<DraftRound>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let current = use_state(|| 0usize);
    let Props { onstage, onchange, rounds } = props.clone();

    let onback = callback!(onstage; move |_| onstage.emit(Stage::Quiz));
    let ondone = callback!(onstage, onchange; move |_| {
        onstage.emit(Stage::Summary);
        onchange.emit(RoundsAction::Submit);
    });

    let list = {
        let onselect = callback!(current; move |idx| current.set(idx));
        let onadd = callback!(current, onchange; move |_| {
            onchange.emit(RoundsAction::Add(*current));
            current.set(*current + 1);
        });
        let onremove = callback!(current, onchange; move |_| {
            onchange.emit(RoundsAction::Remove(*current));
            current.set(current.min(1) - 1);
        });

        let images: Vec<_> = rounds
            .iter()
            .map(|round| round.image.as_ref().map(|x| x.src(Resolution::Thumbnail)))
            .collect();

        html! {<RoundList {onselect} {onadd} {onremove} {images} current={*current}/>}
    };

    let edit = {
        let draft = rounds[*current].clone();
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
