use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &Props) -> Html {
    let classes = classes!("label", &props.extra);

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
