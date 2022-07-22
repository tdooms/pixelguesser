use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub src: Rc<String>,

    #[prop_or(100)]
    pub height: u32,

    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DynImage)]
pub fn dyn_image(props: &Props) -> Html {
    let style = format!("height:{}vh;background-image:url('{}');background-size:contain;background-repeat:no-repeat;background-position:center center;", props.height, props.src);

    html! { <div class={props.class.clone()} {style}> </div> }
}
