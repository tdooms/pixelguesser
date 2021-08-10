#![allow(clippy::redundant_closure_call)]

use yew::prelude::*;
use yewtil::NeqAssign;

use crate::classify;
use crate::common::{TileCtx, TileSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TileProps {
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
    pub vertical: bool,

    /// The size to assign to this tile element.
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub size: Option<TileSize>,
}

/// A single tile element to build 2-dimensional whatever-you-like grids.
///
/// [https://bulma.io/documentation/layout/tiles/](https://bulma.io/documentation/layout/tiles/)
pub struct Tile {
    props: TileProps,
}

impl Component for Tile {
    type Message = ();
    type Properties = TileProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let TileProps { vertical, .. } = self.props;

        let classes = classes!(
            "tile",
            &self.props.extra,
            self.props.ctx.as_ref().map(ToString::to_string),
            self.props.size.as_ref().map(ToString::to_string),
            classify!(vertical)
        );

        html! {
            <@{ self.props.tag.clone() } class={classes}>
                { for self.props.children.iter() }
            </@>
        }
    }
}
