use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props<T: Routable + PartialEq + 'static> {
    children: Children,
    route: T
}

#[function_component(NavbarLink)]
pub fn navbar_link<T: Routable + PartialEq + 'static>(props: &Props<T>) -> Html {
    html! {
        <Link<T> classes={classes!("navbar-link")} route={props.route.clone()}>
            { for props.children.iter() }
        </Link<T>>
    }
}