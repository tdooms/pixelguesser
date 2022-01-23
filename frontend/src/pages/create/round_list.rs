use yew::prelude::*;

use cobul::props::ColumnSize;
use cobul::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<usize>,
    pub images: Vec<Option<String>>,
    pub current: usize,
}

#[function_component(RoundList)]
pub fn round_list(props: &Props) -> Html {
    let map_view = |(index, src): (usize, &Option<String>)| {
        let image = match src {
            Some(src) => html! { <DynImage src={src.clone()} height=10/> },
            None => html! {},
        };

        let grey = (index == props.current).then(|| "has-background-white-ter");
        let onclick = props.onclick.reform(move |_| index);

        html! {
            <div class={classes!("columns", grey)} onclick={onclick}>
                <Column size={ColumnSize::IsNarrow} extra={"m-2 p-2"}> <p> {index} </p> </Column>
                <Column extra="p-1"> {image} </Column>
            </div>
        }
    };

    html! {
        <div class="p-4" style="scrollbar-width:thin;overflow-y:auto">
            { for props.images.iter().enumerate().map(map_view) }
        </div>
    }
}
