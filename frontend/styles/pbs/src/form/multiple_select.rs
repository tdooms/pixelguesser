use crate::properties::{Color, Focused, Hovered, Loading, Rounded, Size};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub hovered: Hovered,

    #[prop_or_default]
    pub focussed: Focused,

    pub selected: Vec<T>,
}

#[function_component(MultipleSelect)]
pub fn multiple_select<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static>(
    props: &Props<T>,
) -> Html {
    let classes = classes!("select", "is-multiple", &props.extra, props.color, props.size);

    // TODO: fix vector
    let view_option = |variant: T| {
        let selected = (props.selected.contains(&variant));

        html! { <option selected={selected}> {variant} </option> }
    };

    html! {
        <div class={classes}>
            <select multiple={true} size={T::iter().count().to_string()} class={classes!(props.hovered, props.focussed)}>
                { for T::iter().map(view_option) }
            </select>
        </div>
    }
}
