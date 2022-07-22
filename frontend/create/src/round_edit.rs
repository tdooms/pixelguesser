use cobul::*;
use components::{Sidebar, SidebarAlignment};
use std::rc::Rc;
use validator::ValidationErrors;
use yew::*;

use api::{DraftRound, Image};

use crate::round_form::{RoundForm, RoundInfo};
use crate::round_preview::RoundPreview;

fn generate_error_message(errors: &[Option<ValidationErrors>]) -> Option<String> {
    let sum: usize = errors.iter().filter_map(|x| x.as_ref().map(|y| y.errors().len())).sum();
    match sum {
        0 => None,
        1 => {
            let idx = errors.iter().position(|x| x.is_some()).unwrap();
            let field = errors[idx].as_ref().unwrap().errors().keys().next().unwrap();
            Some(format!("Missing {field} on round {idx}"))
        }
        _ => Some("Multiple missing images or answers".to_owned()),
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub draft: Rc<DraftRound>,
    pub errors: Rc<Vec<Option<ValidationErrors>>>,

    pub onback: Callback<()>,
    pub ondone: Callback<()>,
    pub onedit: Callback<DraftRound>,
}

#[function_component(RoundEdit)]
pub fn round_edit(props: &Props) -> Html {
    let Props { draft, onback, ondone, onedit, errors } = props.clone();

    let center = {
        let cloned = (*draft).clone();
        let onupload = onedit.reform(move |image| DraftRound { image, ..cloned.clone() });

        html! { <RoundPreview draft={draft.clone()} {onupload}/>}
    };

    let right = {
        let cloned = (*draft).clone();
        let edit = move |info| {
            let RoundInfo { answer, points, guesses } = info;
            DraftRound { answer, points, guesses, ..cloned.clone() }
        };

        let cloned = (*draft).clone();
        let onremove =
            onedit.reform(move |_| DraftRound { image: Image::default(), ..cloned.clone() });

        let disabled = errors.iter().any(|x| x.is_some());
        let tooltip = generate_error_message(&&errors);

        let footer = html! {
            <Buttons class="mt-auto px-4 py-2">
                <Button fullwidth=true color={Color::Info} onclick={ondone} {disabled} {tooltip} style="cursor:pointer" class="mr-1">
                    <span> {"Overview"} </span>
                </Button>
                <Button fullwidth=true color={Color::Info} light=true onclick={onback} class="mr-1">
                    <span> {"Quiz"} </span>
                </Button>
            </Buttons>
        };
        html! {
            <Sidebar size={ColumnSize::IsNarrow} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
                <RoundForm {draft} onchange={onedit.reform(edit)} {onremove}/>
            </Sidebar>
        }
    };

    html! {
        <>
            <Column> {center} </Column>
            {right}
        </>
    }
}
