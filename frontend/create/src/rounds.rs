use cobul::Columns;
use std::rc::Rc;
use validator::Validate;

use api::{Quiz, Round};
use yew::*;
use ywt::callback;

use crate::edit::RoundEdit;
use crate::list::RoundList;
use crate::picker::Picker;
use crate::preview::RoundPreview;

use crate::state::Action;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stage: Callback<Stage>,
    pub action: Callback<Action>,
    pub quiz: Rc<Quiz>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { stage, action, quiz } = props.clone();

    let current = use_state(|| 0usize);
    let round = Rc::new(quiz.rounds[*current].clone());

    let validate = |round: &Round| match round.validate() {
        Ok(_) => None,
        Err(err) => Some(err),
    };

    let errors: Rc<Vec<_>> = Rc::new(quiz.rounds.iter().map(validate).collect());
    let complete = errors.iter().all(|x| x.is_none());

    let ondone = callback!(stage, complete; move |_| {
        if !complete {return}
        stage.emit(Stage::Summary);
    });

    let back = callback!(stage; move |_| {
        stage.emit(Stage::Quiz)
    });

    let edit = callback!(action, current; move |round| {
        action.emit(Action::Round(*current, round));
    });

    let onaction = callback!(current, action; move |new| {
        match new {
            Action::Add(_) => current.set(*current + 1),
            Action::Remove(idx) => current.set(*current - current.min((idx <= *current) as usize)),
            Action::Swap(from, to) => action.emit(Action::Swap(from, to)),
            _ => ()
        }
        action.emit(new);
    });

    let select = callback!(current; move |idx| {
        current.set(idx);
    });

    let change = callback!(round, edit; move |image| {
        edit.emit(Rc::new(DraftRound { image, ..(*round).clone() }));
    });

    let center = match round.image.is_empty() {
        true => html! { <Picker {change} narrow=false/> },
        false => html! { <RoundPreview round={round.clone()} edit={edit.clone()}/> },
    };

    html! {
        <Columns>
            <RoundList {onselect} {action} {draft} current={*current} errors={errors.clone()}/>
            {center}
            <RoundEdit {round} {back} {done} {edit} {errors}/>
        </Columns>
    }
}
