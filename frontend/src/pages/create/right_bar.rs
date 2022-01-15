use super::{RoundForm, RoundInfo};
use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::*;
use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub round: Option<RoundInfo>,
    pub onchange: Callback<RoundInfo>,

    pub ondone: Callback<()>,
    pub onback: Callback<()>,
}

#[function_component(RightBar)]
pub fn right_bar(props: &Props) -> Html {
    let Props { round, onchange, ondone, onback } = &props;

    let footer = html! {
        <Buttons extra="mt-auto px-4 py-2">
            <Button fullwidth=true color={Color::Success} light=true onclick={ondone}>
                <Icon icon={"fas fa-arrow-right"}/> <span> {"done"} </span>
            </Button>
            <Button  fullwidth=true color={Color::Danger} light=true onclick={onback}>
                <Icon icon={"fas fa-arrow-left"}/> <span> {"back"} </span>
            </Button>
        </Buttons>
    };

    let form = match round {
        Some(round) => html! { <RoundForm inner={round.clone()} onchange={onchange} /> },
        None => html! { <div/> },
    };

    html! {
        <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Right} extra="p-0" overflow=false footer={footer}>
            { form }
        </Sidebar>
    }
}
