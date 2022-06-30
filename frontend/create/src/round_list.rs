use yew::prelude::*;

use cobul::props::SidebarAlignment;
use cobul::*;
use shared::callback;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<usize>,
    pub onadd: Callback<()>,
    pub onremove: Callback<usize>,

    pub images: Vec<Option<String>>,
    pub incompletes: Vec<bool>,
    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &Props) -> Html {
    let Props { onselect, onadd, onremove, images, current, incompletes } = props;

    let map_view = |(index, src): (usize, &Option<String>)| {
        let image = match src {
            Some(src) => html! { <DynImage src={src.clone()} height=10/> },
            None => html! {},
        };

        let onselect = onselect.reform(move |_| index);

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

        html! {
            <div style="border-width:thin" tabindex="0" class={classes!("columns", background)} onclick={onselect} {onkeydown}>
                <Column size={ColumnSize::IsNarrow} class={"m-2 pl-2 pr-1"}> <p> {index+1} </p> </Column>
                <Column class="p-1"> {image} </Column>
            </div>
        }
    };

    html! {
        <Sidebar size={ColumnSize::Is2} alignment={SidebarAlignment::Left} overflow=true class="p-4">
            { for images.iter().enumerate().map(map_view) }
            <Button fullwidth=true onclick={onadd} class="ml-1">
                <Icon icon={Icons::Plus} size={Size::Large}/>
            </Button>
        </Sidebar>
    }
}
