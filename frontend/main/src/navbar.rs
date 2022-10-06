use cobul::*;
use yew::*;
use yew_router::prelude::Link;

use profile::Menu;
use shared::Route;

#[function_component(MainNavbar)]
pub fn main_navbar() -> Html {
    let brand = html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Overview}>
            <span><b><i>{"Pixelguesser"}</i></b></span>
        </Link<Route>>
    };

    let end = html! { <Menu/> };

    html! {
        <Navbar {brand} burger=true {end} click={Callback::noop()}/>
    }
}
