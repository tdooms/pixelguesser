use std::rc::Rc;

use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Columns, Icon, Icons, Sidebar};
use futures::FutureExt;
use gloo::timers::callback::Timeout;
use yew::prelude::*;

use api::{DraftRound, Image, Resolution};
use shared::{reduce_callback, set_timer, Error, CREATE_LONG_SAVE_TIME, CREATE_SHORT_SAVE_TIME};

use crate::round_form::{RoundForm, RoundInfo};
use crate::round_preview::CenterSpace;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub draft: DraftRound,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onedit: Callback<DraftRound>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let center = {
        let draft = props.ctx().draft;
        let upload = move |image| DraftRound { image: Some(image), ..draft };
        let remove = move |image| DraftRound { image: None, ..draft };

        let onremove = props.onedit.reform(upload);
        let onupload = props.onedit.reform(remove);

        html! { <CenterSpace image={draft.image.clone()} {onremove} {onupload}/>}
    };

    let right = {
        let draft = props.ctx().draft;
        let edit = move |info| {
            let RoundInfo { answer, points, guesses } = info;
            DraftRound { answer, points, guesses, ..draft }
        };

        let Props { onback, ondone, onedit, draft } = props.clone();

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
                <RoundForm inner={draft.into()} onchange={onedit.reform(edit)} />
            </Sidebar>
        }
    };

    html! {
        <>
            <Column size={ColumnSize::Is8}> {center} </Column>
            {right}
        </>
    }
}
