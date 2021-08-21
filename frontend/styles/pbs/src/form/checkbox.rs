use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    /// The `name` attribute for this form element.
    pub name: String,
    /// The controlled value of this form element.
    pub checked: bool,
    /// The callback to be used for propagating changes to this element's value.
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
}

/// The 2-state checkbox in its native format.
///
/// [https://bulma.io/documentation/form/checkbox/](https://bulma.io/documentation/form/checkbox/)
#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let classes = classes!("checkbox", &props.extra);

    let copied = !props.checked;
    let onchange = props.onchange.reform(move |_| copied);

    html! {
        <label class={classes}>
            <input
                type="checkbox"
                checked={props.checked}
                name={props.name.clone()}
                onclick={onchange}
                disabled={props.disabled}
                />
            { for props.children.iter() }
        </label>
    }
}
