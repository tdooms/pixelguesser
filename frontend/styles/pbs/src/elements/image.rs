use yew::prelude::*;

use crate::properties::{ImageSize, Rounded};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub extra: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,

    #[prop_or_default]
    pub rounded: Rounded,

    #[prop_or_default]
    pub src: Option<String>,
}

/// A container for responsive images.
/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
#[function_component(Image)]
pub fn image(props: &Props) -> Html {
    let classes = classes!("image", &props.extra, props.size);

    html! {
        <figure class={classes}>
            <img class={ classes!(props.rounded) } src={ props.src.clone() } />
        </figure>
    }
}
