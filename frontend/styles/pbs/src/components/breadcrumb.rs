use yew::prelude::*;
use yew::utils::NeqAssign;

use crate::{Alignment, Separator, Size};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct BreadcrumbProps {
    /// The `li` child elements of this breadcrumb.
    #[prop_or_default]
    pub children: Children,
    /// extra classes
    #[prop_or_default]
    pub extra: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Option<Size>,
    /// The alignment of this component.
    #[prop_or_default]
    pub alignment: Option<Alignment>,
    /// The separator type to use between breadcrumb segments.
    #[prop_or_default]
    pub separator: Option<Separator>,
}

/// A simple breadcrumb component to improve your navigation experience.
///
/// [https://bulma.io/documentation/components/breadcrumb/](https://bulma.io/documentation/components/breadcrumb/)
pub struct Breadcrumb {
    props: BreadcrumbProps,
}

impl Component for Breadcrumb {
    type Message = ();
    type Properties = BreadcrumbProps;

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
            "breadcrumb",
            self.props.size.as_ref().map(ToString::to_string),
            self.props.alignment.as_ref().map(ToString::to_string),
            self.props.separator.as_ref().map(ToString::to_string),
            &self.props.extra
        );

        html! {
            <nav class={classes} aria-label="breadcrumbs">
                <ul>
                    { for self.props.children.iter() }
                </ul>
            </nav>
        }
    }
}
