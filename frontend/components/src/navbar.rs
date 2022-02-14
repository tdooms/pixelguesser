use yew::prelude::*;
use yew_router::prelude::Link;

use crate::profile::Profile;
use cobul::*;

use shared::Route;

#[function_component(MainNavbar)]
pub fn main_navbar() -> Html {
    let brand = html! {
        <Link<Route> classes={classes!("navbar-item")} to={Route::Overview}>
            <span><b><i>{"Pixelguesser"}</i></b></span>
        </Link<Route>>
    };

    let end = html! { <Profile/> };

    html! {
        <Navbar brand={brand} burger=true end={end} onclick={Callback::noop()}/>
    }
}
