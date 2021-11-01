use yew::prelude::*;
use yew_router::prelude::Link;

use cobul::props::Color;
use cobul::*;

use crate::route::Route;

#[function_component(MainNavbar)]
pub fn main_navbar() -> Html {
    let brand = html! {
        <Link<Route> classes={classes!("navbar-item")} route={Route::Overview}>
            <img src="https://bulma.io/images/bulma-logo.png" width="112" height="28"/>
        </Link<Route>>
    };

    let start = html! {
        <Buttons>
            <Link<Route> classes={classes!("button", "is-primary")} route={Route::Code}>
                <span>{"Quizmaster"}</span>
            </Link<Route>>

            <Link<Route> classes={classes!("button", "is-light")} route={Route::Create}>
                <span>{"Create"}</span>
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
        <Navbar brand={brand} burger=true start={start} end={end} />
    }
}
