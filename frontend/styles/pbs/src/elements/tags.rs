use yew::prelude::*;

use crate::properties::Addons;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// Attach two tags together; this requires that this component wraps two `Tag` components.
    #[prop_or_default]
    pub addons: Addons,
}

/// A container for a list of tags.
///
/// [https://bulma.io/documentation/elements/tag/](https://bulma.io/documentation/elements/tag/)
#[function_component(Tags)]
pub fn tags(props: &Props) -> Html {
    let classes = classes!("tags", &props.extra, props.addons);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
