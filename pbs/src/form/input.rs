use yew::events::InputData;
use yew::prelude::*;
use yewtil::NeqAssign;

use crate::common::InputType;
use crate::Size;
use crate::{classify, Color};

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct InputProps {
    /// The callback to be used for propagating changes to this element's value.
    pub oninput: Callback<String>,

    /// The `name` attribute for this form element.
    #[prop_or_default]
    pub name: Option<String>,
    /// The controlled value of this form element.
    #[prop_or_default]
    pub value: Option<String>,

    #[prop_or_default]
    pub extra: String,
    /// The input type of this component.
    #[prop_or_default]
    pub r#type: InputType,
    /// The placeholder value for this component.
    #[prop_or_default]
    pub placeholder: String,
    /// The size of this component.
    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub color: Option<Color>,
    /// Use rounded appearance.
    #[prop_or_default]
    pub rounded: bool,
    /// Display a loading spinner within this component.
    #[prop_or_default]
    pub loading: bool,
    /// Disable this component.
    #[prop_or_default]
    pub disabled: bool,
    /// Make this component read-only.
    #[prop_or_default]
    pub readonly: bool,
    /// Make this component static.
    #[prop_or_default]
    pub r#static: bool,
}

/// A text input element.
///
/// [https://bulma.io/documentation/form/input/](https://bulma.io/documentation/form/input/)
///
/// All YBC form components are controlled components. This means that the value of the field must
/// be provided from a parent component, and changes to this component are propagated to the parent
/// component via callback.
pub struct Input {
    props: InputProps,
    link: ComponentLink<Self>,
}

impl Component for Input {
    type Message = String;
    type Properties = InputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.oninput.emit(msg);
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let InputProps {
            rounded,
            loading,
            r#static,
            ..
        } = self.props;

        let classes = classes!(
            "input",
            &self.props.extra,
            self.props.size.to_string(),
            self.props.color.as_ref().map(ToString::to_string),
            classify!(rounded, loading, r#static)
        );

        html! {
            <input
                name=self.props.name.clone()
                value=self.props.value.clone()
                oninput=self.link.callback(|input: InputData| input.value)
                class=classes
                type=self.props.r#type.to_string()
                placeholder=self.props.placeholder.clone()
                disabled=self.props.disabled
                readonly=self.props.readonly
                />
        }
    }
}
