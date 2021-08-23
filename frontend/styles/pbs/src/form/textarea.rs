use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

use crate::properties::{Color, Fixed, Loading, Size, Static};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// The `name` attribute for this form element.
    #[prop_or_default]
    pub name: Option<String>,

    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<String>,

    /// The callback to be used for propagating changes to this element's value.
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub extra: String,

    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,

    /// The number of rows to which this component will be locked.
    #[prop_or_default]
    pub rows: Option<u32>,

    /// The size of this component.
    #[prop_or_default]
    pub size: Size,

    /// The size of this component.
    #[prop_or_default]
    pub color: Option<Color>,

    /// Fix the size of this component.
    #[prop_or_default]
    pub fixed: Fixed,

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

/// A multiline textarea component.
///
/// [https://bulma.io/documentation/form/textarea/](https://bulma.io/documentation/form/textarea/)
#[function_component(Textarea)]
pub fn textarea(props: &Props) -> Html {
    let classes = classes!(
        "textarea",
        &props.extra,
        props.color,
        props.size,
        props.loading,
        props.r#static,
        props.fixed
    );
    let oninput =
        props.oninput.reform(|e: InputEvent| e.target_unchecked_into::<HtmlInputElement>().value());

    html! {
        <textarea
            name={props.name.clone()}
            value={props.value.clone()}
            oninput={oninput}
            class={classes}
            rows={props.rows.as_ref().map(ToString::to_string)}
            placeholder={props.placeholder.clone()}
            disabled={props.disabled}
            readonly={props.readonly}
            />
    }
}
