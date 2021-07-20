use crate::{Color, Size};
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct ProgressProps {
    #[prop_or_default]
    pub extra: String,
    /// The maximum amount of progress; the 100% value.
    #[prop_or_else(|| 1.0)]
    pub max: f32,
    /// The amount of progress which has been made.
    #[prop_or_default]
    pub value: Option<f32>,

    #[prop_or_default]
    pub color: Option<Color>,

    #[prop_or_default]
    pub size: Size,
}

/// A native HTML progress bar.
///
/// [https://bulma.io/documentation/elements/progress/](https://bulma.io/documentation/elements/progress/)
pub struct Progress {
    props: ProgressProps,
}

impl Component for Progress {
    type Message = ();
    type Properties = ProgressProps;

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
            "progress",
            &self.props.extra,
            self.props.size.to_string(),
            self.props.color.as_ref().map(ToString::to_string)
        );

        let max = self.props.max.to_string();
        let value = self.props.value.as_ref().map(ToString::to_string);

        html! {
            <progress class=classes max=max value=value>
                // { format!("{}%", self.props.value) }
            </progress>
        }
    }
}
