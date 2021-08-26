use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub extra: String,
}

#[function_component(DropdownDivider)]
pub fn dropdown_divider(props: &Props) -> Html {
    let classes = classes!("dropdown-divider", &props.extra);
    html! { <hr class={classes} /> }
}