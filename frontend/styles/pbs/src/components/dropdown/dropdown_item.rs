use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: Routable + PartialEq> {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub extra: String,

    route: T
}

#[function_component(DropdownItem)]
pub fn dropdown_item<T: Routable + PartialEq>(props: &Props<T>) -> Html {
    let classes = classes!("dropdown-item", &props.extra);
    html! {
        <Link<T> classes={classes} route={ctx.props().route}>
            { for props.children.iter() }
        </Link<T>>
    }
}