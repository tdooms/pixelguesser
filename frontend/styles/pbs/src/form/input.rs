use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

use crate::properties::{Color, InputType, Loading, Rounded, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// The callback to be used for propagating changes to this element's value.
    pub oninput: Callback<String>,

    /// The `name` attribute for this form element.
    #[prop_or_default]
    pub name: Option<String>,
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub extra: String,
    /// The input type of this component.
    #[prop_or_default]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: Rounded,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: Loading,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: Static,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let classes = classes!(
        "input",
        &props.extra,
        props.size,
        props.color,
        props.rounded,
        props.loading,
        props.r#static,
    );

    let oninput =
        props.oninput.reform(|e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value());

    html! {
        <input
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={oninput}
            class={classes}
            type={props.r#type.to_string()}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
