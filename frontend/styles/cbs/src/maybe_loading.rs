use yew::prelude::*;

use pbs::properties::Color;

use crate::loading::Loading;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub html: Option<Html>,

    #[prop_or_default]
    pub color: Option<Color>,
}

#[function_component(MaybeLoading)]
pub fn maybe_loading(props: &Props) -> Html {
    match &props.html {
        Some(html) => html.clone(),
        None => html! { <Loading color={props.color}/> },
    }
}
