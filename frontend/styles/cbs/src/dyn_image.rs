use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub src: String,

    #[prop_or(100)]
    pub height: u32,
}

#[function_component(DynImage)]
pub fn dyn_image(props: &Props) -> Html {
    let style = format!("height:{}vh;background-image:url('{}');background-size:contain;background-repeat:no-repeat;background-position:center center;", props.height, props.src);

    html! { <div style={style}> </div> }
}
