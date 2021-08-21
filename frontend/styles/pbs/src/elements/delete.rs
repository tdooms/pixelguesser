use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "button".into())]
    pub tag: String,

    pub onclick: Callback<()>,
}

/// A versatile delete cross.
/// [https://bulma.io/documentation/elements/delete/](https://bulma.io/documentation/elements/delete/)
#[function_component(Delete)]
pub fn delete(props: &Props) -> Html {
    let classes = classes!("delete", &props.extra);
    let onclick = props.onclick.reform(|_| ());

    html! {
        <@{props.tag.clone()} class={classes} onclick={onclick}>
            { for props.children.iter() }
        </@>
    }
}
