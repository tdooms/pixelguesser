use yew::prelude::*;
use strum::IntoEnumIterator;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static> {
    #[prop_or_default]
    pub extra: String,

    pub value: T,

    pub onclick: Callback<T>,
}

#[function_component(PanelTabs)]
pub fn panel_tabs<T: IntoEnumIterator + ToString + Copy + PartialEq + 'static>(props: &Props<T>) -> Html {
    let button_map = |variant: T| {
        let active = props.value == variant;
        let onclick = props.onclick.reform(move |_| variant);

        html! {
            <a onclick={onclick} class={active.then(|| "is-active")}>
                {variant.to_string()}
            </a>
        }
    };

    html! {
        <div class={classes!("panel-tabs", &props.extra)}>
            { for T::iter().map(button_map) }
        </div>
    }
}