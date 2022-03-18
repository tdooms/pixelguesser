use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::{Button, Buttons, Column, Icon, Icons, Sidebar};

use yew::prelude::*;

use api::DraftRound;
use shared::Error;

use crate::round_form::{RoundForm, RoundInfo};
use crate::round_preview::RoundPreview;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub draft: DraftRound,
    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onedit: Callback<DraftRound>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { draft, onback, ondone, onedit } = props.clone();

    let center = {
        let clone = draft.clone();
        let upload = move |image| DraftRound { image: Some(image), ..clone.clone() };

        let clone = draft.clone();
        let remove = move |_| DraftRound { image: None, ..clone.clone() };

        let onremove = onedit.reform(remove);
        let onupload = onedit.reform(upload);

        html! { <RoundPreview image={draft.image.clone()} {onremove} {onupload}/>}
    };

    let right = {
        let clone = draft.clone();
        let edit = move |info| {
            let RoundInfo { answer, points, guesses } = info;
            DraftRound { answer, points, guesses, ..clone.clone() }
        };

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

        let inner: RoundInfo = (&draft).into();

        html! {
            <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
                <RoundForm {inner} onchange={onedit.reform(edit)} />
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
