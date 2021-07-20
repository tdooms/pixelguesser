#![allow(clippy::redundant_closure_call)]

use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

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
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub ctx: Option<TileCtx>,
    /// Stack tiles vertically.
    ///
    /// https://bulma.io/documentation/layout/tiles/#modifiers
    #[prop_or_default]
    pub vertical: bool,
    /// The size to assign to this tile element.
    ///
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
        let mut classes = Classes::from("tile");
        if let Some(extra) = &self.props.classes {
            classes = classes.extend(extra);
        }
        if let Some(ctx) = &self.props.ctx {
            classes.push(&ctx.to_string());
        }
        if self.props.vertical {
            classes.push("is-vertical");
        }
        if let Some(size) = &self.props.size {
            classes.push(&size.to_string());
        }
        html! {
            <@{self.props.tag.clone()} class=classes>
                {self.props.children.clone()}
            </@>
        }
    }
}
