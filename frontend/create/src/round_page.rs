use cobul::Columns;
use std::rc::Rc;
use validator::Validate;

use yew::*;

use api::DraftRound;
use ywt::callback;

use crate::round_edit::RoundEdit;
use crate::round_list::RoundList;
use crate::state::RoundsAction;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onaction: Callback<RoundsAction>,
    pub rounds: Rc<Vec<DraftRound>>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { onstage, onaction, rounds } = props.clone();

    let current = use_state(|| 0usize);
    let draft = Rc::new(rounds[*current].clone());

    let validate = |draft: &DraftRound| match draft.validate() {
        Ok(_) => None,
        Err(err) => Some(err),
    };

    let errors: Rc<Vec<_>> = Rc::new(rounds.iter().map(validate).collect());
    let complete = errors.iter().all(|x| x.is_none());

    let ondone = callback!(onstage, complete; move |_| {
        if !complete {return}
        onstage.emit(Stage::Summary);
    });

    let onback = callback!(onstage; move |_| {
        onstage.emit(Stage::Quiz)
    });

    let onedit = callback!(onaction, current; move |round| {
        onaction.emit(RoundsAction::Edit(*current, round));
    });

    let onaction = callback!(current, onaction; move |action| {
        match action {
            RoundsAction::Add(_) => current.set(*current + 1),
            RoundsAction::Remove(idx) => current.set(*current - current.min((idx <= *current) as usize)),
            RoundsAction::Swap(from, to) => onaction.emit(RoundsAction::Swap(from, to)),
            _ => ()
        }
        onaction.emit(action);
    });

    let onselect = callback!(current; move |idx| {
        current.set(idx);
    });

    html! {
        <Columns>
            <RoundList {onselect} {onaction} {rounds} current={*current} errors={errors.clone()} flash=true/>
            <RoundEdit {draft} {onback} {ondone} {onedit} {errors}/>
        </Columns>
    }
}
