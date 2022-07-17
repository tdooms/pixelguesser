use cobul::custom::{Sidebar, SidebarAlignment};
use cobul::*;
use std::rc::Rc;
use validator::ValidationErrors;

use crate::state::RoundsAction;
use api::{DraftRound, Resolution};
use web_sys::DragEvent;
use yew::*;
use ywt::{callback, clone};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ElemProps {
    pub onaction: Callback<RoundsAction>,
    pub onselect: Callback<usize>,

    pub error: bool,
    pub flash: bool,

    pub index: usize,
    pub current: usize,
    pub max: usize,

    pub rounds: Rc<Vec<DraftRound>>,

    pub start: UseStateHandle<Option<usize>>,
    pub hover: UseStateHandle<Option<usize>>,
}

#[function_component(RoundElem)]
pub fn round_elem(props: &ElemProps) -> Html {
    let ElemProps { onaction, onselect, error, flash, index, current, max, rounds, start, hover } =
        props.clone();

    let onclick = onselect.reform(move |_| index);

    let ondragstart = callback!(start; move |event: DragEvent| {
        event.data_transfer().unwrap().set_drop_effect("move");

        start.set(Some(index))
    });
    let ondragover = callback!(hover; move |event: DragEvent| {
        event.prevent_default();
        event.data_transfer().unwrap().set_drop_effect("move");

        hover.set(Some(index))
    });
    let ondragend = callback!(start, hover, onaction; move |event: DragEvent| {
        event.prevent_default();

        if let (Some(start), Some(hover), false) = (*start, *hover, *start == *hover) {
            onaction.emit(RoundsAction::Swap(start, hover));
        }

        start.set(None);
        hover.set(None);
    });
    let ondrop = callback!(; move |event: DragEvent| {
        event.prevent_default();
    });

    let onkeydown = callback!(onaction, onselect; move |e: KeyboardEvent| {
        match (e.key().as_str(), index) {
            ("Delete", _) => onaction.emit(RoundsAction::Remove(index)),
            ("ArrowUp", idx) if idx >= max - 2 => onselect.emit(idx + 1),
            ("ArrowDown", x) if x != 0 => onselect.emit(x - 1),
            _ => ()
        }
    });

    let background = match (error, index == current) {
        (true, false) => "has-background-danger-light",
        (_, true) => "has-background-white-ter",
        (false, false) => "",
    };

    let img_style = "height:100px;display:block;margin-left:auto;margin-right:auto;border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
    let div_style = "border-width:thin";

    let style = match flash {
        true => format!("animation-name:flash-danger;animation-duration:4s;{}", div_style),
        false => div_style.to_string(),
    };

    let class = classes!(background, "columns", "m-0", "p-0");
    let src = rounds[index].image.src(Resolution::Thumbnail);

    html! {
        <div {style} {class} draggable="true" tabindex="0" {onclick} {onkeydown} {ondragstart} {ondragover} {ondragend} {ondrop}>
            <Column size={ColumnSize::IsNarrow} class="m-1 p-0">
                <p> {index+1} </p>
            </Column>

            <Column style="justify-content:center" class="p-1 is-flex">
                <img src={Rc::from(src.as_str())} class="m-0 p-0" style={img_style}/>
            </Column>
        </div>
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ListProps {
    pub onaction: Callback<RoundsAction>,
    pub onselect: Callback<usize>,

    pub rounds: Rc<Vec<DraftRound>>,
    pub errors: Rc<Vec<Result<(), ValidationErrors>>>,

    pub flash: bool,
    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &ListProps) -> Html {
    let ListProps { onaction, onselect, rounds, current, errors, flash } = props.clone();
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

    let max = rounds.len();
    let onclick = onaction.reform(move |_| RoundsAction::Add(current));

    let part = move |index| {
        clone!(start, rounds, hover, onaction, onselect);
        let result: &Result<(), ValidationErrors> = &errors[index];
        let error = result.is_err();

        html! {
            <>
            { draw_line(Some(index) != line_idx) }
            <RoundElem {onaction} {onselect} {error} {flash} {index} {current} {max} {rounds} {start} {hover}/>
            </>
        }
    };

    html! {
        <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} overflow=true class="p-4">
            { for (0..max).map(part) }
            { draw_line(Some(max) != line_idx) }
            <hr class="my-0" />
            <Button fullwidth=true {onclick} class="ml-1">
                <Icon icon={Icons::Plus} size={Size::Large}/>
            </Button>
        </Sidebar>
    }
}
