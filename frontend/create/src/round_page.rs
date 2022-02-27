use std::rc::Rc;

use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Columns, Icon, Icons, Sidebar};
use futures::FutureExt;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

use api::{DraftRound, Image, Resolution};
use shared::{reduce_callback, set_timer, Error, CREATE_LONG_SAVE_TIME, CREATE_SHORT_SAVE_TIME};

use crate::round_form::{RoundForm, RoundInfo};
use crate::round_list::RoundList;
use crate::round_preview::CenterSpace;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub rounds: Vec<DraftRound>,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onsave: Callback<(u64, DraftRound)>,
}

#[function_component(RoundPage)]
pub fn round_page(props: &Props) -> Html {
    let local = use_state(|| None);
    let current = use_state(|| 0_usize);

    let save = Callback::from(|| {});

    let left = {
        let images: Vec<_> = state
            .local
            .iter()
            .map(|x| x.image.as_ref().map(|x| x.src(Resolution::Thumbnail)))
            .collect();

        let onadd = reduce_callback(&state, |_| Action::AddRound);
        let ondelete = reduce_callback(&state, |_| Action::DeleteRound);
        let onselect = reduce_callback(&state, Action::SelectRound);

        html! {
            <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} class="p-0" overflow=true>
                <RoundList {images} {onselect} {ondelete} {onadd} current={state.current}/>
            </Sidebar>
        }
    };

    let center = {
        let onupload = reduce_callback(&state, Action::UploadImage);
        let onremove = reduce_callback(&state, |_| Action::RemoveImage);

        html! { <CenterSpace image={round.image.clone()} {onremove} {onupload}/>}
    };

    let right = {
        let onback = ctx.props().onback.clone();
        let onchange = reduce_callback(&state, Action::UpdateRound);
        let onchange = reduce_callback(&state, |_| Action::Save);
        let round: RoundInfo = round.into();

        let footer = html! {
            <Buttons class="mt-auto px-4 py-2">
                <Button fullwidth=true color={Color::Primary} onclick={ondone} light=true>
                    <Icon icon={Icons::ArrowRight}/> <span> {"Overview"} </span>
                </Button>
                <Button fullwidth=true color={Color::Info} outlined=true onclick={onback}>
                    <Icon icon={Icons::ArrowLeft}/> <span> {"Edit Quiz"} </span>
                </Button>
            </Buttons>
        };
        html! {
            <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
                <RoundForm inner={round.clone()} onchange={onchange} />
            </Sidebar>
        }
    };

    html! {
        <Columns>
            {left}
            <Column size={ColumnSize::Is8}> {center} </Column>
            {right}
        </Columns>
    }
}
