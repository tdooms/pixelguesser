use cobul::Columns;
use std::rc::Rc;
use validator::Validate;

use yew::*;

use api::{DraftQuiz, DraftRound};
use ywt::callback;

use crate::edit::RoundEdit;
use crate::list::RoundList;
use crate::picker::Picker;

use crate::state::Action;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub onstage: Callback<Stage>,
    pub onaction: Callback<Action>,
    pub draft: Rc<DraftQuiz>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    log::info!("round page");
    let Props { onstage, onaction, draft } = props.clone();

    let current = use_state(|| 0usize);
    let round = Rc::new(draft.rounds.data[*current].clone());

    let validate = |round: &DraftRound| match round.validate() {
        Ok(_) => None,
        Err(err) => Some(err),
    };

    let errors: Rc<Vec<_>> = Rc::new(draft.rounds.data.iter().map(validate).collect());
    let complete = errors.iter().all(|x| x.is_none());

    let ondone = callback!(onstage, complete; move |_| {
        if !complete {return}
        onstage.emit(Stage::Summary);
    });

    let onback = callback!(onstage; move |_| {
        onstage.emit(Stage::Quiz)
    });

    let onedit = callback!(onaction, current; move |round| {
        onaction.emit(Action::Round(*current, round));
    });

    let onaction = callback!(current, onaction; move |action| {
        match action {
            Action::Add(_) => current.set(*current + 1),
            Action::Remove(idx) => current.set(*current - current.min((idx <= *current) as usize)),
            Action::Swap(from, to) => onaction.emit(Action::Swap(from, to)),
            _ => ()
        }
        onaction.emit(action);
    });

    let onselect = callback!(current; move |idx| {
        current.set(idx);
    });

    html! {
        <Columns>
            <RoundList {onselect} {onaction} {draft} current={*current} errors={errors.clone()}/>
            // <RoundPreview round={round.clone()} onedit={onedit.clone()}/>
            <Picker onchange={Callback::noop()} />
            <RoundEdit {round} {onback} {ondone} {onedit} {errors}/>
        </Columns>
    }
}
