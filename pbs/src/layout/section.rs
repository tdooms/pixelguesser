use crate::common::SectionSize;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
    /// A size modifier to control spacing.
    #[prop_or_default]
    pub size: Option<SectionSize>,
}

/// A simple container to divide your page into sections.
///
/// [https://bulma.io/documentation/layout/section/](https://bulma.io/documentation/layout/section/)
pub struct Section {
    props: SectionProps,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;

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
            "section",
            &self.props.extra,
            self.props.size.as_ref().map(ToString::to_string)
        );

        html! {
            <section class=classes>
                { for self.props.children.iter() }
            </section>
        }
    }
}
