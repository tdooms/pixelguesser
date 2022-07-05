use std::cmp::Ordering;
use std::rc::Rc;
use web_sys::DragEvent;
use yew::prelude::*;

use crate::state::RoundsAction;
use cobul::props::SidebarAlignment;
use cobul::*;
use shared::callback;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<usize>,
    pub onadd: Callback<()>,
    pub onremove: Callback<usize>,
    pub onswap: Callback<(usize, usize)>,

    pub images: Vec<Rc<String>>,
    pub incompletes: Vec<bool>,
    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &Props) -> Html {
    let Props { onselect, onadd, onremove, onswap, images, current, incompletes } = props;
    let original = use_state_eq(|| None);
    let hover = use_state_eq(|| None);

    let visualise = |hidden: bool| {
        html! {
            <hr class="p-0 m-0 has-background-danger" style={hidden.then_some("visibility:hidden")}/>
        }
    };

    let line = match (*original, *hover) {
        (Some(original), Some(hover)) if original > hover => Some(hover),
        (Some(original), Some(hover)) if original < hover => Some(hover + 1),
        _ => None,
    };

    let map_view = |(index, src): (usize, &Rc<String>)| {
        let onselect = onselect.reform(move |_| index);

        let ondragstart = callback!(original; move |event: DragEvent| {
            event.data_transfer().unwrap().set_drop_effect("move");
            original.set(Some(index))
        });
        let ondragover = callback!(hover; move |event: DragEvent| {
            event.prevent_default();
            event.data_transfer().unwrap().set_drop_effect("move");
            hover.set(Some(index))
        });

        let ondragend = callback!(original, hover, onswap; move |event: DragEvent| {
            event.prevent_default();
            if *original != *hover {
                onswap.emit((original.unwrap(), hover.unwrap()));
            }

            original.set(None);
            hover.set(None);
        });

        let ondrop = callback!(; move |event: DragEvent| {
            event.prevent_default();
        });

        let onkeydown = callback!(onremove; move |e: KeyboardEvent| {
            if e.key() == "Delete" {
                onremove.emit(index);
            }
        });

        let background = match (incompletes[index], index == *current) {
            (true, false) => "has-background-danger-light",
            (_, true) => "has-background-white-ter",
            (false, false) => "",
        };

        let img_style = "height:100px;display:block;margin-left:auto;margin-right:auto;border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
        let style = "border-width:thin";
        let class = classes!(background, "columns", "m-0", "p-0");

        html! {
            <>
            { visualise(Some(index) != line) }
            <div {style} {class} draggable="true" tabindex="0" onclick={onselect} {onkeydown} {ondragstart} {ondragover} {ondragend} {ondrop}>
                <Column size={ColumnSize::IsNarrow} class="m-1 p-0">
                    <p> {index+1} </p>
                </Column>

                <Column style="justify-content:center" class="p-1 is-flex">
                    <img src={(**src).clone()} class={"m-0 p-0"} style={img_style}/>
                </Column>
            </div>
            </>
        }
    };

    html! {
        <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} overflow=true class="p-4">
            { for images.iter().enumerate().map(map_view) }
            { visualise(Some(images.len()) != line) }
            <Button fullwidth=true onclick={onadd} class="ml-1">
                <Icon icon={Icons::Plus} size={Size::Large}/>
            </Button>
        </Sidebar>
    }
}
