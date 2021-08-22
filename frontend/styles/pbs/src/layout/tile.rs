use yew::prelude::*;

use crate::properties::{TileCtx, TileSize, Vertical};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    /// The HTML tag to use for this component.
    #[prop_or_else(|| "div".into())]
    pub tag: String,

    /// The context modifier to use for this tile element, else none.
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub ctx: Option<TileCtx>,

    /// Stack tiles vertically.
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: Vertical,

    /// The size to assign to this tile element.
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
#[function_component(Tile)]
pub fn tile(props: &Props) -> Html {
    let classes = classes!("tile", &props.extra, props.ctx, props.size, props.vertical);

    html! {
        <@{ props.tag.clone() } class={classes}>
            { for props.children.iter() }
        </@>
    }
}
