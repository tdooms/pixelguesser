use super::{RoundForm, RoundInfo};
use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::*;
use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub round: Option<RoundInfo>,
    pub onchange: Callback<RoundInfo>,
    pub size: ColumnSize,

    pub ondone: Callback<()>,
    pub onback: Callback<()>,
}

#[function_component(RightBar)]
pub fn right_bar(props: &Props) -> Html {
    let Props { round, onchange, ondone, onback, size } = &props;

    let footer = html! {
        <Buttons class="mt-auto px-4 py-2">
            <Button fullwidth=true color={Color::Primary} onclick={ondone}>
                <Icon icon={Icons::ArrowRight}/> <span> {"next"} </span>
            </Button>
            <Button  fullwidth=true color={Color::Danger} light=true onclick={onback}>
                <Icon icon={Icons::ArrowLeft}/> <span> {"back"} </span>
            </Button>
        </Buttons>
    };

    let form = match round {
        Some(round) => html! { <RoundForm inner={round.clone()} onchange={onchange} /> },
        None => html! { <div/> },
    };

    html! {
        <Sidebar size={size.clone()} alignment={SidebarAlignment::Right} class="p-0" overflow=false footer={footer}>
            { form }
        </Sidebar>
    }
}
