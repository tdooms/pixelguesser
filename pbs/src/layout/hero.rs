use yew::prelude::*;
use yewtil::NeqAssign;

use crate::{Color, HeroSize};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HeroProps {
    /// Extra classes for the hero container.
    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Option<HeroSize>,

    #[prop_or_default]
    pub header: Option<Html>,

    #[prop_or_default]
    pub body: Option<Html>,

    #[prop_or_default]
    pub footer: Option<Html>,
}

/// An imposing hero banner to showcase something.
///
/// [https://bulma.io/documentation/layout/hero/](https://bulma.io/documentation/layout/hero/)
pub struct Hero {
    props: HeroProps,
}

impl Component for Hero {
    type Message = ();
    type Properties = HeroProps;

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
        let classes = classes!(
            "hero",
            self.props.size.as_ref().map(ToString::to_string),
            self.props.color.as_ref().map(ToString::to_string),
            &self.props.extra
        );

        // TODO: fix code duplication
        let header = match &self.props.header {
            Some(html) => html! { <div class="hero-header"> {html.clone()} </div> },
            None => html! {},
        };

        let body = match &self.props.body {
            Some(html) => html! { <div class="hero-body"> {html.clone()} </div> },
            None => html! {},
        };

        let footer = match &self.props.footer {
            Some(html) => html! { <div class="hero-footer"> {html.clone()} </div> },
            None => html! {},
        };

        html! {
            <section class={classes}>
                {header} {body} {footer}
            </section>
        }
    }
}
