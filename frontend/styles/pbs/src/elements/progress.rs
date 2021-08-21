use crate::properties::{Color, Size};
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub extra: String,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_default]
    pub value: Option<f32>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
#[function_component(Image)]
pub fn image(props: &Props) -> Html {
    let classes = classes!("progress", &props.extra, props.size, props.color);

    let max = props.max.to_string();
    let value = props.value.as_ref().map(ToString::to_string);

    html! {
        <progress class={classes} max={max} value={value}>
            // { format!("{}%", self.props.value) }
        </progress>
    }
}
