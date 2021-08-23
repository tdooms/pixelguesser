use pbs::prelude::*;
use pbs::properties::Color;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub extra: String,

    #[prop_or_default]
    pub label: Option<String>,

    #[prop_or_default]
    pub help: Option<String>,

    #[prop_or_default]
    pub help_color: Option<Color>,

    #[prop_or_default]
    pub icon_right: Option<String>,

    #[prop_or_default]
    pub icon_left: Option<String>,
}

#[function_component(SimpleField)]
pub fn simple_field(props: &Props) -> Html {
    let help = match &props.help {
        Some(help) => {
            html! { <Help color={props.help_color}> {help.clone()} </Help> }
        }
        None => html! {},
    };

    let label = match &props.label {
        Some(label) => html! {<Label> {label.clone()} </Label>},
        None => html! {},
    };

    let right = match &props.icon_right {
        Some(right) => html! {<Icon icon={right.clone()} extra="is-right"/>},
        None => html! {},
    };

    let left = match &props.icon_left {
        Some(left) => html! {<Icon icon={left.clone()} extra="is-left"/>},
        None => html! {},
    };

    let control_classes = classes!(
        "control",
        props.icon_right.as_ref().map(|_| "has-icons-right"),
        props.icon_left.as_ref().map(|_| "has-icons-left")
    );

    html! {
        <div class={classes!("field", props.extra.clone())}>
            { label }
            <div class={control_classes}>
                { right }
                { left }
                { for props.children.iter() }
            </div>
            { help }
        </div>
    }
}
