use std::rc::Rc;

use cobul::*;
use shared::{callback, clone};
use validator::ValidationErrors;
use web_sys::DragEvent;
use yew::*;

use api::{Quiz, Resolution};
use components::{DynImage, Fit, Height, Sidebar, SidebarAlignment};

use crate::state::Action;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ElemProps {
    pub action: Callback<Action>,
    pub select: Callback<usize>,

    pub error: bool,

    pub index: usize,
    pub current: usize,
    pub max: usize,

    pub quiz: Rc<Quiz>,

    pub start: UseStateHandle<Option<usize>>,
    pub hover: UseStateHandle<Option<usize>>,
}

#[function_component(RoundElem)]
pub fn round_elem(props: &ElemProps) -> Html {
    let ElemProps { action, select, error, index, current, max, quiz, start, hover } =
        props.clone();

    let onclick = select.reform(move |_| index);

    let ondragstart = callback!(start; move |event: DragEvent| {
        event.data_transfer().unwrap().set_drop_effect("move");
        start.set(Some(index))
    });
    let ondragover = callback!(hover; move |event: DragEvent| {
        event.prevent_default();
        event.data_transfer().unwrap().set_drop_effect("move");
        hover.set(Some(index))
    });
    let ondragend = callback!(start, hover, action; move |event: DragEvent| {
        event.prevent_default();

        if let (Some(start), Some(hover), false) = (*start, *hover, *start == *hover) {
            action.emit(Action::Swap(start, hover));
        }

        start.set(None);
        hover.set(None);
    });
    let ondrop = callback!(move |event: DragEvent| {
        event.prevent_default();
    });

    let onkeydown = callback!(action, select; move |e: KeyboardEvent| {
        match (e.key().as_str(), index) {
            ("Delete", _) => action.emit(Action::Remove(index)),
            ("ArrowUp", idx) if idx >= max - 2 => select.emit(idx + 1),
            ("ArrowDown", x) if x != 0 => select.emit(x - 1),
            _ => ()
        }
    });

    let background = match (error, index == current) {
        (true, false) => "has-background-danger-light",
        (_, true) => "has-background-white-ter",
        (false, false) => "",
    };

    let class = classes!(background, "columns", "m-0", "p-0");
    let style = "border-width:thin";
    let src = quiz.rounds[index].image.src(Resolution::Thumb);
    let placeholder = quiz.rounds[index].image.blurhash();

    html! {
        <div {style} {class} draggable="true" tabindex="0" {onclick} {onkeydown} {ondragstart} {ondragover} {ondragend} {ondrop}>
            <Column size={ColumnSize::IsNarrow} class="m-1 p-0">
                <p> {index+1} </p>
            </Column>

            <Column>
            <DynImage {src} {placeholder} height={Height::Px(100)} fit={Fit::Cover} border=true/>
            </Column>
        </div>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ListProps {
    pub action: Callback<Action>,
    pub select: Callback<usize>,

    pub quiz: Rc<Quiz>,
    pub errors: Rc<Vec<ValidationErrors>>,

    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &ListProps) -> Html {
    let ListProps { action, select, quiz, current, errors } = props.clone();
    let start = use_state_eq(|| None);
    let hover = use_state_eq(|| None);

    let draw_line = |hidden: bool| {
        html! { <hr class="p-0 m-0 has-background-danger" style={hidden.then_some("visibility:hidden")}/> }
    };

    let line_idx = match (*start, *hover) {
        (Some(start), Some(hover)) if start > hover => Some(hover),
        (Some(start), Some(hover)) if start < hover => Some(hover + 1),
        _ => None,
    };

    let max = quiz.rounds.len();
    let click = action.reform(move |_| Action::Add(current));

    let part = move |index: usize| {
        clone!(start, hover, action, select, quiz);
        let error = errors.get(index) != Some(&ValidationErrors::default());

        html! {
            <>
            { draw_line(Some(index) != line_idx) }
            <RoundElem {action} {select} {error} {index} {current} {max} {quiz} {start} {hover} />
            </>
        }
    };

    html! {
        <Sidebar style="width:230px" alignment={SidebarAlignment::Left} overflow=true class="p-4">
            { for (0..max).map(part) }
            { draw_line(Some(max) != line_idx) }
            <hr class="my-0" />
            <simple::Button fullwidth=true {click} class="ml-1" icon={fa::Solid::Plus} />
        </Sidebar>
    }
}
