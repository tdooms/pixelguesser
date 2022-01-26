use super::RoundList;
use cobul::props::{Color, ColumnSize, SidebarAlignment};
use cobul::*;
use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct Props {
    pub images: Vec<Option<String>>,
    pub current: usize,
    pub size: ColumnSize,

    pub onremove: Callback<()>,
    pub onadd: Callback<()>,
    pub onselect: Callback<usize>,
}

#[function_component(LeftBar)]
pub fn left_bar(props: &Props) -> Html {
    let Props { images, current, onremove, onadd, onselect, size } = &props;

    let footer = html! {
        <Buttons class="mt-auto px-4 my-2">
            <Button fullwidth=true color={Color::Success} light=true onclick={onadd}>
                <Icon icon={Icons::Plus}/> <span> {"add round"} </span>
            </Button>
            <Button fullwidth=true color={Color::Danger} light=true onclick={onremove}>
                <Icon icon={Icons::Trash}/> <span> {"delete round"} </span>
            </Button>
        </Buttons>
    };

    html! {
        <Sidebar size={size.clone()} alignment={SidebarAlignment::Left} class="p-0" overflow=false footer={footer}>
            <RoundList images={images.clone()} onclick={onselect} current={*current}/>
        </Sidebar>
    }
}
