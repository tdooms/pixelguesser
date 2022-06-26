use yew::prelude::*;

use cobul::props::{ColumnSize, Size};
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onselect: Callback<usize>,
    pub onadd: Callback<()>,
    pub onremove: Callback<()>,

    pub images: Vec<Option<String>>,
    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &Props) -> Html {
    let Props { onselect, onadd, onremove, images, current } = props;

    let map_view = |(index, src): (usize, &Option<String>)| {
        let image = match src {
            Some(src) => html! { <DynImage src={src.clone()} height=10/> },
            None => html! {},
        };

        let grey = (index == *current).then(|| "has-background-white-ter");

        let onselect = onselect.reform(move |_| index);
        let onremove = onremove.reform(move |_| ());

        html! {
            <div class={classes!("columns", grey)} onclick={onselect}>
                <Column size={ColumnSize::IsNarrow} class={"m-2 pl-2 pr-1"}> <p> {index} </p> </Column>
                <Column class="p-1"> {image} </Column>

                <div style="position:relative;right:3px;top:10px"> <Delete onclick={onremove}/> </div>
            </div>
        }
    };

    html! {
        <Column class="p-4">
            { for images.iter().enumerate().map(map_view) }
            <Button fullwidth=true onclick={onadd}>
                <Icon icon={Icons::Plus} size={Size::Large}/>
            </Button>
        </Column>
    }
}
