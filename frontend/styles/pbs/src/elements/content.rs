use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,
    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,
}

/// A single component to wrap WYSIWYG generated content, where only HTML tags are available.
/// [https://bulma.io/documentation/elements/content/](https://bulma.io/documentation/elements/content/)
#[function_component(Content)]
pub fn content(props: &Props) -> Html {
    let classes = classes!("content", &props.extra);
    html! {
        <@{props.tag.clone()} class={classes}>
            { for props.children.iter() }
        </@>
    }
}
