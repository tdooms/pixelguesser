use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::classify;
use crate::ImageSize;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub extra: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,

    #[prop_or_default]
    pub rounded: bool,

    #[prop_or_default]
    pub src: Option<String>,
}

/// A container for responsive images.
///
/// [https://bulma.io/documentation/elements/image/](https://bulma.io/documentation/elements/image/)
pub struct Image {
    props: ImageProps,
}

impl Component for Image {
    type Message = ();
    type Properties = ImageProps;

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
        let classes =
            classes!("image", &self.props.extra, self.props.size.as_ref().map(ToString::to_string));

        let ImageProps { rounded, .. } = self.props;

        html! {
            <figure class={classes}>
                <img class={classes!(classify!(rounded))} src={ self.props.src.clone() } />
            </figure>
        }
    }
}
