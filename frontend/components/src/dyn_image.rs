use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Height {
    Vh(u32),
    Px(u32),
}

impl Default for Height {
    fn default() -> Self {
        Height::Vh(100)
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub src: Rc<String>,

    #[prop_or_default]
    pub height: Height,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: String,

    #[prop_or_default]
    pub fit: Fit,

    #[prop_or_default]
    pub border: bool,
}

#[derive(derive_more::Display, Debug, PartialEq, Clone, Copy, Default)]
pub enum Fit {
    #[default]
    #[display(fmt = "none")]
    None,
    #[display(fmt = "contain")]
    Contain,
    #[display(fmt = "cover")]
    Cover,
    #[display(fmt = "scale-down")]
    ScaleDown,
    #[display(fmt = "fill")]
    Fill,
}

#[function_component(DynImage)]
pub fn dyn_image(props: &Props) -> Html {
    let height = match props.height {
        Height::Vh(vh) => format!("height:{}vh", vh),
        Height::Px(px) => format!("height:{}px", px),
    };

    let border = "border-width:thin;border-style:solid;border-radius:5px;border-color:lightgray";
    let fit = format!("object-fit:{};display:block;margin-left:auto;margin-right:auto", props.fit);
    let style = format!("{height};{fit};{}", props.border.then(|| border).unwrap_or_default());

    html! {
        <div style="justify-content:center" class="p-0 m-0 is-flex">
        <img src={(*props.src).clone()} class="m-0 p-0" {style}/>
        </div>
    }
}
