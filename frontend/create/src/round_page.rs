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

    let errors: Vec<_> = rounds.iter().map(|x| x.validate()).collect();
    let complete = errors.iter().all(|x| x.is_ok());

    let ondone = callback!(onstage, onaction, complete; move |_| {
        if !complete {return}
        onstage.emit(Stage::Summary);
        onaction.emit(RoundsAction::Submit);
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

    let (current, errors) = (*current, Rc::new(errors));
    html! {
        <Columns>
            <RoundList {onselect} {onaction} {rounds} {current} {errors} flash=true/>
            <RoundEdit {draft} {onback} {ondone} {onedit} {complete}/>
        </Columns>
    }
}
