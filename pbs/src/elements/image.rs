use crate::ImageSize;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ImageProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<ImageSize>,
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
        let classes = classes!(
            "image",
            &self.props.extra,
            self.props.size.as_ref().map(ToString::to_string)
        );
        html! {
            <figure class=classes>
                { for self.props.children.iter() }
            </figure>
        }
    }
}
