use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub name: String,

    #[prop_or_default]
    pub checked: Option<T>,
}

#[function_component(Radio)]
pub fn radio<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static>(
    props: &Props<T>,
) -> Html {
    let classes = classes!("box", &props.extra);

    let view_box = |variant: T| {
        let checked = Some(variant) == props.checked;
        html! { <label class="radio"> <input type="radio" name={props.name.clone()} /> {variant} </label> }
    };

    html! { { for T::iter().map(view_box) } }
}
