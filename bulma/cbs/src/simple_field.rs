use yew::prelude::*;
use yewtil::NeqAssign;

use pbs::Color;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct SimpleFieldProps {
    #[prop_or_default]
    children: Children,

    #[prop_or_default]
    extra: String,

    #[prop_or_default]
    label: Option<String>,

    #[prop_or_default]
    help: Option<String>,

    #[prop_or_default]
    help_color: Option<Color>,

    #[prop_or_default]
    icon_right: Option<String>,

    #[prop_or_default]
    icon_left: Option<String>,
}

pub struct SimpleField {
    props: SimpleFieldProps,
}

impl Component for SimpleField {
    type Message = ();
    type Properties = SimpleFieldProps;

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
        let help = match &self.props.help {
            Some(help) => {
                html! { <pbs::Help color={self.props.help_color}> {help.clone()} </pbs::Help> }
            }
            None => html! {},
        };

        let label = match &self.props.label {
            Some(label) => html! {<pbs::Label> {label.clone()} </pbs::Label>},
            None => html! {},
        };

        let right = match &self.props.icon_right {
            Some(right) => html! {<pbs::Icon icon={right.clone()} extra="is-right"/>},
            None => html! {},
        };

        let left = match &self.props.icon_left {
            Some(left) => html! {<pbs::Icon icon={left.clone()} extra="is-left"/>},
            None => html! {},
        };

        let control_classes = classes!(
            "control",
            self.props.icon_right.then(|| "has-icons-right"),
            self.props.icon_left.then(|| "has-icons-left")
        );

        html! {
            <div class=classes!("field", self.props.extra.clone())>
                { label }
                <div class=control_classes>
                    { right }
                    { left }
                    { for self.props.children.iter() }
                </div>
                { help }
            </div>
        }
    }
}
