use yew::prelude::*;
use yew_router::prelude::Link;

use cobul::props::Color;
use cobul::*;

use crate::route::Route;

#[function_component(MainNavbar)]
pub fn main_navbar() -> Html {
    let brand = html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Overview}>
            <span>{"Pixelguesser"}</span>
        </Link<Route>>
    };

    let start = html! {
        <Buttons>
            <Link<Route> classes={classes!("button", "is-light")} to={Route::Create}>
                <span>{"Create"}</span>
            </Link<Route>>

            <Link<Route> classes={classes!("button", "is-light")} to={Route::Test}>
                <span>{"Test"}</span>
            </Link<Route>>
        </Buttons>
    };

    let end = html! {
        <Buttons>
            <Button color={Color::Primary}> <strong>{"Sign up"}</strong> </Button>
            <Button light={true}> {"Log in"} </Button>
        </Buttons>
    };

    html! {
        <Navbar brand={brand} burger=true start={start} end={end} onclick={Callback::noop()}/>
    }
}
