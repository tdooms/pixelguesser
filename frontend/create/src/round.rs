use std::rc::Rc;

use cobul::Columns;
use validator::Validate;
use yew::*;
use ywt::callback;

use api::{Quiz, Round};

use crate::edit::RoundEdit;
use crate::list::RoundList;
use crate::picker::Picker;
use crate::preview::RoundPreview;
use crate::state::Action;
use crate::Stage;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub change: Callback<Stage>,
    pub action: Callback<Action>,
    pub quiz: Rc<Quiz>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let Props { change, action, quiz } = props.clone();

    let current = use_state(|| 0usize);
    let round = Rc::new(quiz.rounds[*current].clone());

    let validate = |round: &Round| match round.validate() {
        Ok(_) => Default::default(),
        Err(err) => err,
    };

    let errors: Rc<Vec<_>> = Rc::new(quiz.rounds.iter().map(validate).collect());
    let complete = errors.iter().all(|x| x.is_empty());

    let done = callback!(change, complete; move |_| {
        if !complete {return}
        change.emit(Stage::Summary);
    });

    let back = callback!(change; move |_| {
        change.emit(Stage::Quiz)
    });

    let edit = callback!(action, current; move |round| {
        action.emit(Action::Round(*current, round));
    });

    let action = callback!(current, action; move |new| {
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

    let input = callback!(round, edit; move |image| {
        edit.emit(Rc::new(Round { image, ..(*round).clone() }));
    });

    let center = match round.image.is_empty() {
        true => html! { <Picker change={input} narrow=false /> },
        false => html! { <RoundPreview round={round.clone()} edit={edit.clone()} /> },
    };

    html! {
        <Columns>
            <RoundList {select} {action} {quiz} current={*current} errors={errors.clone()}/>
            {center}
            <RoundEdit {round} {back} {done} input={edit} {errors}/>
        </Columns>
    }
}
