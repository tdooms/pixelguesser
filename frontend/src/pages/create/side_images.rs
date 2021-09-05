use yew::prelude::*;

use pbs::prelude::*;
use pbs::properties::ColumnSize;

pub enum Msg {
    Clicked(usize),
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    pub onclick: Callback<usize>,
    pub images: Vec<Option<String>>,
    pub current: usize,
}

#[function_component(SideImages)]
pub fn side_images(props: &Props) -> Html {
    let map_view = |(index, src): (usize, &Option<String>)| {
        let image = match src {
            Some(src) => html! { <cbs::DynImage src={src.clone()} height=10/> },
            None => html! {},
        };

        let grey = (index == props.current).then(|| "has-background-white-ter");
        let onclick = props.onclick.reform(move |_| index);

        html! {
            <div class={classes!("columns", grey)} onclick={onclick}>
                <Column size={ColumnSize::IsNarrow}> <p> {index} </p> </Column>
                <Column extra="p-1"> {image} </Column>
            </div>
        }
    };

    html! {
        <div class="p-4" style="overflow-y:auto">
            { for props.images.iter().enumerate().map(map_view) }
        </div>
    }
}
